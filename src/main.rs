#[cfg(not(debug_assertions))]
use axum::extract::Request;
use axum::Router;
#[cfg(not(debug_assertions))]
use hyper::body::Incoming;
#[cfg(not(debug_assertions))]
use hyper_util::rt::TokioIo;
#[cfg(not(debug_assertions))]
use std::path::PathBuf;
use std::sync::Arc;
use tera::Tera;
use tokio::sync::RwLock;
#[cfg(not(debug_assertions))]
use tower::ServiceExt;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// モジュール宣言
mod models;
mod state;
mod routes {
    pub mod api;
    pub mod dashboard;
    pub mod users;
}

use state::{AppState, SharedState};
#[cfg(not(debug_assertions))]
use tracing::error;
use tracing::info;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env file not found");

    // -------------------------------------------------------------------------
    // ■ ロガーの初期化 (ここから)
    // -------------------------------------------------------------------------
    // 1. ファイル出力設定 (logsディレクトリに、app.log.YYYY-MM-DD で毎日ローテーション)
    let file_appender = tracing_appender::rolling::daily("logs", "app.log");
    // 2. 非ブロッキング書き込み (Webサーバーの性能を落とさないため)
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    // 3. ログレベル設定 (デフォルトは info 以上を表示)
    // RUST_LOG環境変数で "debug" や "warn" に上書き可能
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "my_cms=info,tower_http=info,sqlx=warn".into());
    // 4. フォーマット設定 (コンソール用とファイル用を合成)
    tracing_subscriber::registry()
        .with(env_filter)
        // コンソール出力層
        .with(tracing_subscriber::fmt::layer())
        // ファイル出力層 (色は付けない)
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_writer(non_blocking),
        )
        .init();
    // -------------------------------------------------------------------------
    // ■ ロガーの初期化 (ここまで)
    // -------------------------------------------------------------------------

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = models::init_pool(&database_url)
        .await
        .expect("Failed to connect to MySQL");

    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            info!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let state: SharedState = Arc::new(AppState {
        tera: RwLock::new(tera),
        pool,
    });

    // ■ ルーティングの構築
    let app = Router::new()
        .merge(routes::dashboard::routes())
        .nest("/users", routes::users::routes())
        .nest("/api", routes::api::routes())
        .nest_service("/static", ServeDir::new("public"))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // 開発環境 (cargo run): TCPポート 3000 で起動
    #[cfg(debug_assertions)]
    {
        let port = 3000;
        let addr = format!("0.0.0.0:{}", port);
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

        info!("development mode: listening on http://{}", addr);

        axum::serve(listener, app).await.unwrap();
    }
    // 本番環境 (cargo run --release): Unixドメインソケットで起動
    #[cfg(not(debug_assertions))]
    {
        let socket_path = PathBuf::from("/tmp/my_cms.sock");
        if socket_path.exists() {
            std::fs::remove_file(&socket_path).unwrap();
        }

        let listener = tokio::net::UnixListener::bind(&socket_path).unwrap();
        info!("production mode: listening on unix:{:?}", socket_path);

        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&socket_path, std::fs::Permissions::from_mode(0o777)).unwrap();

        loop {
            let (socket, _remote_addr) = match listener.accept().await {
                Ok(s) => s,
                Err(_) => continue,
            };
            let app = app.clone();
            tokio::spawn(async move {
                let socket = TokioIo::new(socket);
                let hyper_service = hyper::service::service_fn(move |req: Request<Incoming>| {
                    let app = app.clone();
                    app.oneshot(req)
                });
                if let Err(err) = hyper_util::server::conn::auto::Builder::new(
                    hyper_util::rt::TokioExecutor::new(),
                )
                .serve_connection(socket, hyper_service)
                .await
                {
                    error!("failed to serve connection: {:#}", err);
                }
            });
        }
    }
}
