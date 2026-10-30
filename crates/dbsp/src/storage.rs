//! Storage APIs for Feldera.
//!
//! The API consists of a lower layer, [backend], that provides block-based
//! access to storage, a middle layer, [buffer_cache], that implements
//! buffering, and an upper layer, [mod@file], that implements data access.
pub mod backend;
pub mod buffer_cache;
#[cfg(unix)]
pub mod dirlock;

// wasm32 spike: storage is never configured, so a directory lock is never taken.
#[cfg(not(unix))]
pub mod dirlock {
    use super::backend::StorageError;
    use std::path::Path;
    use std::time::Duration;

    #[derive(Debug)]
    pub struct LockedDirectory;

    impl LockedDirectory {
        pub fn new_blocking<P: AsRef<Path>>(_: P, _: Duration) -> Result<LockedDirectory, StorageError> {
            Err(StorageError::StorageDisabled)
        }
        pub fn new<P: AsRef<Path>>(_: P) -> Result<LockedDirectory, StorageError> {
            Err(StorageError::StorageDisabled)
        }
        pub fn base(&self) -> &Path {
            Path::new("")
        }
    }
}
pub mod file;
pub mod tracking_bloom_filter;

#[cfg(not(target_arch = "wasm32"))]
use fdlimit::{Outcome::LimitRaised, raise_fd_limit};
use tracing::warn;

use std::sync::Once;

/// Raise the fd limit to run the storage library to avoid surprises.  This
/// is a no-op on Windows, so it does not need to be behind an architectural
/// cfg.
#[cfg(target_arch = "wasm32")]
fn init_fd_limit() {}

#[cfg(not(target_arch = "wasm32"))]
fn init_fd_limit() {
    match raise_fd_limit() {
        Ok(LimitRaised { from, to }) => {
            const WARN_THRESHOLD: u64 = 1 << 19;
            if to < WARN_THRESHOLD {
                warn!(
                    "Raised fd limit from {} to {}. It's still very low -- try increasing the fd hard-limit (in your limits.conf).",
                    from, to
                );
            }
        }
        Ok(_) => { /* not on unix */ }
        Err(e) => {
            warn!("Failed to raise fd limit: {}", e);
        }
    }
}

/// Performs storage module initialization.
///
/// On Unix, this raises the process's maximum file descriptor from its "soft"
/// to "hard" limit.  On Windows, this has no effect.
///
/// This function is idempotent and initializing any of the storage backends
/// will call it automatically.  It can log, so initialize logging before
/// calling it.
pub fn init() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        init_fd_limit();
    });
}
