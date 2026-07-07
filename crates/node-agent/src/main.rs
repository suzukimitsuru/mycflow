//! ノードエージェント（スタブ）。
//!
//! 将来の責務:
//! - wkg管理下のOCIレジストリからWASMコンポーネントをpull
//! - WAMR（組み込み向け）/ wasmtime（コンテナ向け）でのモジュール起動
//! - `mycflow:node/lifecycle` によるリソース報告を配置マネージャーへ中継
//!
//! 現時点ではDocker初期環境でのプロセス起動確認のみを行うスタブ。

fn main() -> anyhow::Result<()> {
    println!("mycflow node-agent (stub) starting");
    Ok(())
}
