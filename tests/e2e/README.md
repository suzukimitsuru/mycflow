# tests/e2e/

`deploy/docker/docker-compose.yml` の環境を起動し、`compositions/` で定義した固定DAG
（例: `sensor-to-aggregator`）を実際に流してみるE2E統合テスト置き場。

## 想定フロー

1. `docker compose -f deploy/docker/docker-compose.yml up -d`
2. `placement-manager` が `compositions/*.topology.json` をetcdへ投入
3. `node-a` / `node-b` がコンポーネントを起動し、Zenoh経由でチャネル通信
4. 期待した集約結果が得られることを確認

現時点ではコンポーネント実装前のため、スクリプト（`run.sh`等）は未作成。
`components/` と `compositions/` の実体化後に追加する。
