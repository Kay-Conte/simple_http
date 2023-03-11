use std::any::{Any, TypeId};
use std::collections::HashMap;

type MapValue = Box<dyn Any + Sync + Send + 'static>;

/// A HashMap wrapper capable of storing Type Value pairs where there is one instance of each type.
pub struct AnyMap {
    inner: HashMap<TypeId, MapValue>,
}

impl AnyMap {
    /// Construct a new `AnyMap` with an empty inner map.
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    /// Inserts a value giving a generic as a key and a value of the same type.
    pub fn insert<K: Any + Sync + Send + 'static>(&mut self, value: K) -> Option<K> {
        let type_id = value.type_id();
        self.inner
            .insert(type_id, Box::new(value))
            .map(|k| *k.downcast::<K>().expect("Invalid type"))
    }

    /// Gets a value given a generic as a key.
    pub fn get<K: Any + Sync + Send + 'static>(&self) -> Option<&K> {
        self.inner
            .get(&TypeId::of::<K>())
            .map(|k| k.downcast_ref::<K>().expect("Invalid type"))
    }

    /// Gets a value mutable given a generic as a key.
    pub fn get_mut<K: Any + Sync + Send + 'static>(&mut self) -> Option<&mut K> {
        self.inner
            .get_mut(&TypeId::of::<K>())
            .map(|k| k.downcast_mut::<K>().expect("Invalid type"))
    }

    /// Remove an item from the map and returns it given a generic as a key.
    pub fn remove<K: Any + Sync + Send + 'static>(&mut self) -> Option<K> {
        self.inner
            .remove(&TypeId::of::<K>())
            .map(|k| *k.downcast::<K>().expect("Invalid type"))
    }
}
