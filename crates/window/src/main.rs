mod apps;
use apps::app::create_app;

/// アプリケーションを起動するエントリーポイント。
fn main() {
    create_app();
}
