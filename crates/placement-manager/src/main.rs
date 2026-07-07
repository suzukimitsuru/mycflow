//! 配置マネージャー（スタブ）。
//!
//! 将来の責務:
//! - etcd上の配置メタデータ・トポロジグラフの管理
//! - CPU/メモリ負荷・通信量・モジュール転送コストに基づく配置判断
//! - トポロジグラフからのZenoh ACL生成（config/zenoh/ の雛形を実体化する）
//!
//! 現時点ではDocker初期環境でのプロセス起動確認のみを行うスタブ。

fn main() -> anyhow::Result<()> {
    println!("mycflow placement-manager (stub) starting");
    println!("etcd placement prefix: {}", mycflow_common::ETCD_PLACEMENT_PREFIX);
    println!("etcd topology prefix:  {}", mycflow_common::ETCD_TOPOLOGY_PREFIX);
    Ok(())
}
