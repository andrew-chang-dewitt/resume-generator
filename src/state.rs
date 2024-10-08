use std::{collections::HashMap, hash::Hash};

use async_trait::async_trait;

use crate::model;

#[derive(Debug)]
pub struct AppState {
    resume: TempCache<model::Resume>,
    contact: TempCache<model::Contact>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            resume: TempCache::new(),
            contact: TempCache::new(),
        }
    }
}
// create a single type to encapsulate all State behaviours
pub trait State<V>: Create<V> + Get<V> {}

// then impl that type automatically for anything that impls all the behaviours
// this greatly simplifies fn/type signatures for things that use/refer to States
impl<S, V> State<V> for S where S: Create<V> + Get<V> {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    Tmp(i64),
    Db(i64),
}

pub trait Map: Clone {
    type Item;
    type Output;

    fn map<F: Fn(&Self::Item) -> Self::Output>(&self, f: F) -> Self::Output;
}

impl Map for Key {
    type Item = i64;
    type Output = i64;

    fn map<F: Fn(&Self::Item) -> Self::Output>(&self, f: F) -> Self::Output {
        match self {
            Self::Tmp(val) => f(val),
            Self::Db(val) => f(val),
        }
    }
}

/// Add a new value to an implementing structure, returning the values new id number.
pub trait Create<V> {
    fn create(&mut self, value: V) -> Key;
}

#[async_trait]
pub trait Get<V> {
    async fn get(&self, key: &Key) -> Option<&V>;
}

impl Create<model::Resume> for AppState {
    fn create(&mut self, value: model::Resume) -> Key {
        self.resume.create(value)
    }
}

impl Create<model::Contact> for AppState {
    fn create(&mut self, value: model::Contact) -> Key {
        self.contact.create(value)
    }
}

#[async_trait]
impl Get<model::Resume> for AppState {
    async fn get(&self, key: &Key) -> Option<&model::Resume> {
        self.resume.get(key).await
    }
}

#[async_trait]
impl Get<model::Contact> for AppState {
    async fn get(&self, key: &Key) -> Option<&model::Contact> {
        self.contact.get(key).await
    }
}

/// A db cache separating values found from the db (cache) from values that have yet to be saved
/// to the db (temp).
#[derive(Debug)]
struct TempCache<V> {
    next_tmp_key: i64,
    cache: HashMap<Key, V>,
}

impl<V> TempCache<V> {
    /// Inits an empty cache w/ no values in either HashMap.
    fn new() -> Self {
        Self {
            next_tmp_key: Default::default(),
            cache: HashMap::new(),
        }
    }

    fn len(&self) -> usize {
        self.cache.len()
    }
}

#[cfg(test)]
#[test]
fn new_temp_cache_starts_empty() {
    let t = TempCache::<String>::new();

    assert_eq!(t.len(), 0, "{t:?} shouldn't have any items in it yet.");
}

impl<V> Create<V> for TempCache<V> {
    /// Add a new value to the cache.  Creates new items with a temp key, allowing them to be added to the
    /// database & the cache updated with their actual id value on save.
    fn create(&mut self, value: V) -> Key {
        let key = Key::Tmp(self.next_tmp_key);
        self.cache.insert(key, value);
        self.next_tmp_key;
        key
    }
}

#[cfg(test)]
#[test]
fn temp_cache_can_create_new_values() {
    let mut t = TempCache::<String>::new();
    t.create("This is a new value".into());

    assert_eq!(t.len(), 1, "{t:?} should have exactly one item in it.");
}

#[async_trait]
impl<V: Sync> Get<V> for TempCache<V> {
    /// Extract a value with a matching id from the cache, or the underlying data store if not
    /// found in cache (updating the cache when found), or None if no matching value is found.
    async fn get(&self, key: &Key) -> Option<&V> {
        // first we check if the requested id is in the cache
        if let Some(value) = self.cache.get(key) {
            return Some(value);
        } else {
            // TODO: actually get this from the db instead
            None
        }
        // finally, if not found in either, we check the db & load it into the cache if found
    }
}

#[cfg(test)]
#[tokio::test]
async fn can_get_items_from_temp_cache() {
    let mut t = TempCache::<String>::new();
    let id = t.create("This is a new value".into());
    let gotten = t.get(&id).await;

    assert_eq!(gotten, Some(&"This is a new value".into()));
}
