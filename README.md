# mycflow

Mycelium Flow Environment

菌糸ネットワークの自己組織化からインスピレーションを得た、自律的なモジュール配置・実行環境。
設計の詳細は [docs/01 mycflow 基本概念.md](docs/01%20mycflow%20基本概念.md) / [docs/02 mycflow 機能設計.md](docs/02%20mycflow%20機能設計.md) を参照。
開発の進め方は [docs/03 mycflow 開発計画.md](docs/03%20mycflow%20開発計画.md)、設計判断の経緯は [docs/adr/](docs/adr/) を参照。

## リポジトリ構成

```text
mycflow/
├── docs/           # 設計md・ADR
├── wit/            # WITパッケージ定義（一次情報。wkgでOCIレジストリに公開する単位）
├── crates/         # Rustワークスペース（配置マネージャー・ホスト関数層・ノードエージェント）
├── components/     # リファレンスWASMコンポーネント（動作検証用）
├── compositions/   # wac合成定義 + トポロジグラフ定義（固定DAGパイプライン）
├── deploy/         # docker（初期ターゲット） / k8s / kubeedge（将来用）
├── config/         # zenoh ACLテンプレート、wkg名前空間マッピング
└── tests/e2e/      # compose環境での固定DAG統合テスト
```

現フェーズはWITインターフェース設計中・Docker初期環境が対象（モノレポ構成の理由は[ADR 0001](docs/adr/0001-monorepo-structure.md)参照）。

## 開発

```sh
cargo check --workspace         # crates/ 配下のホスト側実装
wasm-tools component wit wit/   # WIT定義の検証
```
