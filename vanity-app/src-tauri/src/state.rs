use std::sync::Mutex;
use vanity_core::CancelToken;

/// Tracks the single in-flight grind job, if any. Only one grind runs at a time,
/// matching the CLI's interaction model.
#[derive(Default)]
pub struct AppState {
    pub job: Mutex<Option<CancelToken>>,
}
