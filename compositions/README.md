# compositions/

「何が存在するか」（`components/`）と「どう結線するか」（ここ）の分離。

固定DAGパイプラインの定義は次の2つのペアで構成する。

- `*.wac` — [wac](https://github.com/bytecodealliance/wac) によるコンポーネント合成定義（静的強制）。
- `*.topology.json` — 同じDAGをetcdのトポロジグラフへ投入するための定義。配置マネージャーが
  これを読み込み、Zenoh ACLを生成する（動的強制）。両者は同一のDAGを別レイヤーで表現するため、
  必ずペアで追加・更新する。

## 例: sensor-to-aggregator パイプライン

- [`sensor-to-aggregator.wac`](sensor-to-aggregator.wac)
- [`sensor-to-aggregator.topology.json`](sensor-to-aggregator.topology.json)

現時点ではプレースホルダ。`wit/`のインターフェースが固まり次第、`components/`の実装に合わせて実体化する。
