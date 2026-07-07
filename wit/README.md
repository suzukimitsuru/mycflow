# wit/

mycflowのWITインターフェース定義（一次情報）。wkgでOCIレジストリに公開する単位はパッケージ単位。

wasm-toolsの標準レイアウト（トップレベル=ルートパッケージ、依存パッケージは `deps/` 配下）に従う。

- `embedded.wit` / `container.wit` — `mycflow:worlds` パッケージ（ルート）。各実行形態
  （組み込み/コンテナ）向けの world 定義で、`dataflow` と `node` の両インターフェースを
  import/export として組み合わせる。
- `deps/dataflow/` — `mycflow:dataflow` パッケージ。channel resource・open-channel・
  エラー型（permission-denied 等）を定義する。
- `deps/node/` — `mycflow:node` パッケージ。ノードのリソース報告・ライフサイクルを定義する。

`deps/` 配下でも `dataflow` `node` は独立した公開単位であり、`worlds` の付属物ではない
（ディレクトリ配置はWITツーリングの解決規則に合わせただけ）。

## 検証方法

```sh
wasm-tools component wit wit/
```

このディレクトリ全体がパースできることを確認する（CIの「WIT検証」ステージに相当）。

## 公開方針

インターフェースが安定するまでは `wkg publish` の実運用は行わず、`--dry-run` 相当の検証のみCIで回す。
安定後、`deps/dataflow/` `deps/node/` を独立したバージョニング単位として `wkg publish` する。
