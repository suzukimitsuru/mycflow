//! 配置マネージャー・ホスト関数層・ノードエージェント間で共有する型と
//! etcdスキーマ（キー命名規則）を定義する。

use serde::{Deserialize, Serialize};

/// 配置メタデータ（Layer 1）を格納するetcdキーのプレフィックス。
pub const ETCD_PLACEMENT_PREFIX: &str = "/mycflow/placement/";
/// データフロートポロジグラフを格納するetcdキーのプレフィックス。
pub const ETCD_TOPOLOGY_PREFIX: &str = "/mycflow/topology/";

/// ノードを一意に識別するID。
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub String);

/// モジュール（WASMコンポーネント）を一意に識別するID。
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ModuleId(pub String);

/// `open-channel` で参照される論理チャネル名。
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChannelName(pub String);

/// トポロジグラフ上の1本の許可済みデータパス。
/// 配置マネージャーがこれを元にZenoh ACLを生成する。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelRoute {
    pub channel: ChannelName,
    pub from_module: ModuleId,
    pub to_module: ModuleId,
}

/// etcdに書き込む配置メタデータ（どのモジュールがどのノードにいるか）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Placement {
    pub module: ModuleId,
    pub node: NodeId,
}
