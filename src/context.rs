use std::sync::Arc;

use anymap::AnyMap;

pub(crate) struct Context {
    data: Arc<AnyMap>,
}

impl Context {
    pub(crate) fn new() -> Self {
        Self {
            data: Arc::new(AnyMap::new())
        }
    }
}