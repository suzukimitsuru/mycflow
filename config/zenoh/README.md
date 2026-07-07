# config/zenoh/

Zenohのキー式ACLテンプレート置き場。配置マネージャーが `compositions/*.topology.json`
（etcd上のトポロジグラフ）から実際のACLを生成する際の雛形をここに置く。

- `acl-template.json5` — 配置マネージャーが埋め込み値（モジュールID・ノードID）を差し込むテンプレート。

生成されたACLの実体は配置マネージャーの実行時にetcd経由でZenohルーターへ配布される想定であり、
このディレクトリにはコミットしない（雛形のみを管理する）。
