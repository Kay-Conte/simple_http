use std::sync::Arc;

use anymap::AnyMap;

/// Wrapping structure for all application wide data
pub struct Context {
    data: Arc<AnyMap>,
}

impl Context {
    pub(crate) fn new() -> Self {
        Self {
            data: Arc::new(AnyMap::new()),
        }
    }
}
