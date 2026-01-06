# Rust Axum Web Template (No-Node.js / Modular Arch)

Rust (Axum), MySQL (SQLx), そしてスタンドアロンのフロントエンドツール (esbuild/Tailwind) を組み合わせた、パフォーマンスと保守性を重視したWebアプリケーションテンプレートです。
Node.js (npm/node_modules) を一切使用せず、Rustのエコシステムと単一バイナリツールのみで完結する「質実剛健」な構成です。

## 🚀 特徴

* **Backend:** [Axum](https://github.com/tokio-rs/axum) - 高速でモジュラーなWebフレームワーク。
* **Database:** [SQLx](https://github.com/launchbadge/sqlx) - コンパイル時チェック付きの型安全なSQL (MySQL)。
* **Template:** [Tera](https://github.com/Keats/tera) - Jinja2ライクなHTMLテンプレート (`.html.tera`)。
* **Frontend:**
    * **JS:** [esbuild](https://esbuild.github.io/) (CLI) - 爆速バンドラー。
    * **CSS:** [Tailwind CSS](https://tailwindcss.com/) (CLI) - ユーティリティファーストCSS。
* **Logging:** Tracing & Tracing Appender - 日次ローテーション付きの構造化ログ。
* **Architecture:**
    * **Hybrid:** SSR (HTML) と REST API (JSON) の両方を提供。
    * **Modular:** `routes` (Controller) と `models` (DAO) を明確に分離。

## 📂 ディレクトリ構造

```t
.
├── src/
│   ├── main.rs            # エントリーポイント & 設定
│   ├── state.rs           # 共有ステート (DB Pool, Tera)
│   ├── models/            # データアクセス層 (DAO)
│   │   ├── mod.rs         # 共通設定 & モジュール公開
│   │   ├── sensor.rs      # センサー関連クエリ
│   │   └── user.rs        # ユーザー関連クエリ
│   └── routes/            # ルーティング & ハンドラ
│       ├── dashboard.rs   # HTML: トップページ
│       ├── users.rs       # HTML: ユーザー管理
│       └── api/           # JSON API
│           ├── mod.rs     # APIルーターの統合
│           ├── sensor.rs  # /api/sensors
│           └── user.rs    # /api/users
├── templates/             # Teraテンプレート (*.html.tera)
├── public/                # 静的ファイル (CSS/JSの出力先)
├── src_js/                # JSソースコード
├── logs/                  # ログ出力先 (自動生成)
├── .env                   # 環境変数
└── Cargo.toml
```

## 🛠 前提条件
Rust: 最新の安定版 (rustup update)
MySQL: v8.0以上
Tools: curl (フロントエンドツールのダウンロードに使用)

## 🗄️ Database Migrations (SQLx)
DBスキーマの変更は、SQLを手動実行するのではなく、マイグレーションファイルで管理します。

1. ツールのインストール
`sqlx` CLIツールをインストールします（初回のみ）。
```bash
# MySQL機能を有効にしてインストール
cargo install sqlx-cli --no-default-features --features native-tls,mysql
```

2. マイグレーションファイルの作成
スキーマ変更を行う際は、新しいファイルを作成します。
```bash
# migrations/YYYYMMDDHHMMSS_description.sql が生成されます
# -r オプションで revert（ロールバック用）ファイルも同時生成
sqlx migrate add -r create_users_table
```
生成されたSQLファイルにDDLを記述します。
### migrations/xxxx_up.sql: テーブル作成などの変更内容 (CREATE/ALTER)
### migrations/xxxx_down.sql: 取り消し内容 (DROP/REVERT)

3. マイグレーションの適用
作成したSQLをデータベースに反映させます。
```bash
# 未適用のマイグレーションをすべて実行
sqlx migrate run
```

4. 変更の取り消し (Rollback)
直前の変更を取り消したい場合に使用します。
```bash
sqlx migrate revert
```

## ⚙️ セットアップ & 起動
1. データベースの準備
```bash
sqlx migrate run
# DB初期化する場合は
sqlx migrate reset
```

2. 環境変数 (.env)
プロジェクトルートに .env ファイルを作成します。
```bash
DATABASE_URL=mysql://user:password@localhost:3306/my_cms_db
RUST_LOG=my_cms=info,tower_http=info,sqlx=warn
```

3. フロントエンドツールの準備
npmを使わず、単一バイナリをダウンロードして配置します (Linux x64の例)。
```bash
# esbuild の取得
curl -sL [https://registry.npmjs.org/@esbuild/linux-x64/-/linux-x64-0.19.11.tgz](https://registry.npmjs.org/@esbuild/linux-x64/-/linux-x64-0.19.11.tgz) | tar -xz package/bin/esbuild
mv package/bin/esbuild . && rm -rf package

# Tailwind CSS の取得
curl -sL -o tailwindcss [https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64](https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64)
chmod +x tailwindcss
```

4. ビルド & 実行
JSをバンドルし、Rustサーバーを起動します。
```bash
# JSビルド (Minify有効)
./esbuild src_js/app.js --bundle --minify --outfile=public/js/app.js
# CSS自動生成
./tailwindcss -i input.css -o public/css/style.css --watch

# サーバー起動 (ホットリロード開発時は cargo watch -x run が便利)
cargo run
    Web Interface: http://localhost:3000
    API Endpoint: http://localhost:3000/api/sensors
```

## 📡 API利用例
```bash
# センサーデータの登録 (POST)
curl -X POST http://localhost:3000/api/sensors \
  -H "Content-Type: application/json" \
  -d '{"temperature": 36.5, "heart_rate": 72}'

# ユーザーの登録 (POST)
curl -X POST http://localhost:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{"username": "Admin", "email": "admin@example.com"}'
```
## 📝 開発ガイド
機能を拡張する際は、以下のルールに従ってください。

1. テーブル追加:
src/models/new_entity.rs を作成し、構造体とSQLクエリを記述。
src/models/mod.rs に登録。

2. Webページ追加:
テンプレート templates/new_page.html.tera を作成。
ハンドラ src/routes/new_page.rs を作成。
src/main.rs でルーティングを追加。

3. API追加:
ハンドラ src/routes/api/new_entity.rs を作成 (JSONを返す)。
src/routes/api/mod.rs で .nest() する。

## 🚀 モードによる挙動の違い
本プロジェクトは `cargo run` のフラグによって起動モードが切り替わります。

### 開発モード (Development)
- コマンド: `cargo run`
- 通信: **TCP (http://0.0.0.0:3000)**
- 用途: ブラウザでの動作確認
- 機能: `/static` へのアクセスで `public` フォルダを配信します。

### 本番モード (Production)
- コマンド: `cargo run --release`
- 通信: **Unix Domain Socket (/tmp/my_cms.sock)**
- 用途: Nginx 等のリバースプロキシと組み合わせた運用

## 📜 License
MIT