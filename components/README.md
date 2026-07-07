# components/

動作検証用のリファレンスWASMコンポーネント。`crates/`（Rustワークスペース）には含めず、
`wasm32-wasip2` ターゲット + [`cargo-component`](https://github.com/bytecodealliance/cargo-component)
で個別にビルドする（ホスト側バイナリと異なるツールチェーン・ターゲットのため）。

- `sensor-sim/` — センサー値を模擬生成し `mycflow:dataflow/channel` へ送信するコンポーネント。
- `aggregator/` — チャネルを受信し集約するコンポーネント。

## ビルド方法（想定）

```sh
(cd components/sensor-sim && cargo component build --release --target wasm32-wasip2)
(cd components/aggregator && cargo component build --release --target wasm32-wasip2)
```

`wit/embedded.wit` または `wit/container.wit`（`mycflow:worlds` パッケージ）を対象worldとして bindgen する。
`compositions/` のwac定義がこれらのコンポーネントを結線する。
