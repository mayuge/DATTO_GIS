# DATTO_GIS
軽量GIS

## デスクトップ版

```
cd crates\window
cargo run
```

## WebAssembly 版

Rust の WASM ターゲットと Trunk を一度だけインストールします。

```
rustup target add wasm32-unknown-unknown
cargo install trunk
```

開発サーバーを起動すると、ブラウザで地図を操作できます。

```
cd crates\window
trunk serve
```

既定の URL は <http://localhost:8080> です。タイル画像を表示するため、
ブラウザからアクセスできる HTTPS の XYZ タイル URL を使用してください。