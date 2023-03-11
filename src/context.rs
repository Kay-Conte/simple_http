use crate::any_map::AnyMap;

/// Wrapping structure for all application wide data

pub struct Context {
    pub data: AnyMap,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            data: AnyMap::new(),
        }
    }
}

impl Context {
    pub fn new(data: AnyMap) -> Self {
        Self { data }
    }
}
