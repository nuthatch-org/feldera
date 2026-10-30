/// The process running dbsp can share a single Tokio runtime.
///
/// This is `pub` so it can be used in adapters too for IO.
#[cfg(not(target_arch = "wasm32"))]
pub use feldera_storage::tokio::{TOKIO, TOKIO_DEDICATED_IO};

// wasm32 spike: the multihost exchange spawns onto this runtime; a tab has no sockets, so nothing
// is ever spawned, and the type only has to exist.
#[cfg(target_arch = "wasm32")]
pub struct NoTokio;

#[cfg(target_arch = "wasm32")]
impl NoTokio {
    pub fn spawn<F: std::future::Future + 'static>(&self, _: F) {
        panic!("the multihost exchange has no runtime on wasm32");
    }
}

#[cfg(target_arch = "wasm32")]
pub static TOKIO: NoTokio = NoTokio;
