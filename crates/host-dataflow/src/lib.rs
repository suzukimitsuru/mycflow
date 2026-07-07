//! `mycflow:dataflow` の `open-channel` に対応するホスト関数層。
//!
//! WAMR統合（C API境界）とwasmtime統合の両方から呼ばれる想定のため、
//! ランタイム非依存のロジック（許可チェックのフック含む）をここに置く。

use mycflow_common::ChannelName;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DataflowError {
    #[error("permission denied for channel `{0}`")]
    PermissionDenied(String),
    #[error("channel `{0}` not found")]
    ChannelNotFound(String),
}

/// `open-channel` 呼び出し時の許可チェックを差し込むためのフックポイント。
/// 実装は配置マネージャーが生成するトポロジグラフ/Zenoh ACLを参照する想定。
pub trait PermissionChecker: Send + Sync {
    fn is_allowed(&self, caller_module: &str, channel: &ChannelName) -> bool;
}

/// 開発・テスト用の全許可チェッカー。本番の許可チェックはetcd/Zenoh ACL連携版に差し替える。
pub struct AllowAll;

impl PermissionChecker for AllowAll {
    fn is_allowed(&self, _caller_module: &str, _channel: &ChannelName) -> bool {
        true
    }
}

/// `mycflow:dataflow/channel.open-channel` のホスト実装本体。
pub fn open_channel(
    checker: &dyn PermissionChecker,
    caller_module: &str,
    channel: ChannelName,
) -> Result<ChannelName, DataflowError> {
    if checker.is_allowed(caller_module, &channel) {
        Ok(channel)
    } else {
        Err(DataflowError::PermissionDenied(channel.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DenyAll;
    impl PermissionChecker for DenyAll {
        fn is_allowed(&self, _caller_module: &str, _channel: &ChannelName) -> bool {
            false
        }
    }

    #[test]
    fn allowed_caller_opens_channel() {
        let result = open_channel(&AllowAll, "sensor-sim", ChannelName("sensor.raw".into()));
        assert_eq!(result, Ok(ChannelName("sensor.raw".into())));
    }

    #[test]
    fn denied_caller_gets_permission_denied() {
        let result = open_channel(&DenyAll, "sensor-sim", ChannelName("sensor.raw".into()));
        assert_eq!(
            result,
            Err(DataflowError::PermissionDenied("sensor.raw".into()))
        );
    }
}
