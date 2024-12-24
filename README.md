# Wasmを使ってブラウザ上で顔モザイクするアプリ

## RustからWasmファイルにビルドする方法
■注意: こまったらChatGPTに聞きましょう。全て教えてくれます。

### Rustで打つコマンド
- Wasm化するためのツールをインストール
```bash
cargo install wasm-bindgen-cli
```
- Wasmバイナリをビルドします
```bash
cargo build --target wasm32-unknown-unknown --release
```
- JavaScriptで使えるようにバインディングします。
```
wasm-bindgen target/wasm32-unknown-unknown/release/プロジェクト名.wasm --out-dir ./pkg --target web
```
### React（JavaScriptでやること）
- JavaScriptのバインディングファイルからモジュールをインポート
```JavaScript
// 今回はsetup_detectorとdetect_bounding_boxをインポート
import init, { setup_detector, detect_bounding_box } from "./../../wasm/pkg/rustfase_detection";
```

- initしてWasmモジュールを初期化
```JavaScript
await init(); // モジュールを使う前に実行
```
 - （オプション）もし、Wasmモジュールを使わないときはインスタンスを解放したい場合
 - ```JavaScript
    wasmModule = await init(); // 初期化
    wasmModule = null; // 参照を解放 ⇒ いつかJavaScriptのGCで解放される
   ```
