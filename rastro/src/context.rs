use slog::Logger;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use slog_scope::logger;

pub struct Context {
    pub id: String,
    pub logger: Logger
}

static CONTEXT_COUNTER: AtomicUsize = AtomicUsize::new(0);

impl Context {
    pub fn new() -> Self {
        let id = format!("{}", CONTEXT_COUNTER.fetch_add(1, Ordering::Relaxed));
        let logger = logger().new(slog::o!("context" => id.clone()));
        slog::info!(logger, "Starting Context");
        Context { id, logger }
    }
    //
    // fn new_child(&self) -> Self {
    //     let id = format!("{}", CONTEXT_COUNTER.fetch_add(1, Ordering::Relaxed));
    //     let logger = logger().new(slog::o!("context" => id.clone()));
    //     let quit = self.quit.clone();
    //     slog::info!(logger, "Starting Context");
    //     Context { id, logger, quit }
    // }
}

impl Drop for Context {
    fn drop(&mut self) {
        slog::info!(self.logger, "Closing Context")
    }
}
