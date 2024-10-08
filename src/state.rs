use std::{collections::HashMap, hash::Hash};

use async_trait::async_trait;

use crate::model;

/// The basic structure of all data for entire app.
///
/// Everything fits into one of a few specified data types, each with it's own cache.
#[derive(Debug, Clone)]
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
pub trait State<Val, Idx>: AddNew<Val, Idx> + Get<Val, Idx> {}

// then impl that type automatically for anything that impls all the behaviours
// this greatly simplifies fn/type signatures for things that use/refer to States
impl<S, Val, Idx> State<Val, Idx> for S where S: AddNew<Val, Idx> + Get<Val, Idx> {}

/// Most (maybe all?) data types use this key type.
///
/// By differentiating between temporary index values & those from the DB, key collisions
/// are eliminated, yet items are allowed to exist in the cache that haven't been saved to the DB
/// yet without having to know all possible keys that might be in the DB already.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    Tmp(i64),
    Db(i64),
}

/// Add a new value to an implementing structure, returning the values new id number.
pub trait AddNew<Val, Idx> {
    fn add_new(&mut self, value: Val) -> Idx;
}

/// Get a value matching the corresponding key & data type, if it exists.
#[async_trait]
pub trait Get<Val, Idx> {
    async fn get(&self, key: &Idx) -> Option<&Val>;
}

impl AddNew<model::Resume, Key> for AppState {
    fn add_new(&mut self, value: model::Resume) -> Key {
        self.resume.add_new(value)
    }
}

impl AddNew<model::Contact, Key> for AppState {
    fn add_new(&mut self, value: model::Contact) -> Key {
        self.contact.add_new(value)
    }
}

#[async_trait]
impl Get<model::Resume, Key> for AppState {
    async fn get(&self, key: &Key) -> Option<&model::Resume> {
        self.resume.get(key).await
    }
}

#[async_trait]
impl Get<model::Contact, Key> for AppState {
    async fn get(&self, key: &Key) -> Option<&model::Contact> {
        self.contact.get(key).await
    }
}

/// A db cache separating values found from the db (cache) from values that have yet to be saved
/// to the db (temp). Tracks temporary key values in memory, providing value for the next item.
#[derive(Debug, Clone)]
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

    /// A temp cache knows it's length.
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

impl<V> AddNew<V, Key> for TempCache<V> {
    /// Add a new value to the cache.  Creates new items with a temp key, allowing them to be added to the
    /// database & the cache updated with their actual id value on save.
    fn add_new(&mut self, value: V) -> Key {
        let key = Key::Tmp(self.next_tmp_key);
        self.cache.insert(key, value);
        self.next_tmp_key;
        key
    }
}

#[cfg(test)]
#[test]
fn temp_cache_can_add_new_new_values() {
    let mut t = TempCache::<String>::new();
    t.add_new("This is a new value".into());

    assert_eq!(t.len(), 1, "{t:?} should have exactly one item in it.");
}

#[async_trait]
impl<V: Sync> Get<V, Key> for TempCache<V> {
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
    let id = t.add_new("This is a new value".into());
    let gotten = t.get(&id).await;

    assert_eq!(gotten, Some(&"This is a new value".into()));
}
