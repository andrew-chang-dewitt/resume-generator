use std::collections::HashMap;

use async_trait::async_trait;
use sqlx::SqlitePool;

use crate::model;

/// A data store, containing application state & db connection pool for updating underlying
/// persistent data storage.
#[derive(Debug)]
pub struct Store {
    pool: SqlitePool, // not sure this needs to be arc?
    state: State,
}

impl Store {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool,
            state: State::new(),
        }
    }
}

/// Add resumes to store, deferring to state.
impl Add<model::Resume> for Store {
    fn add(&mut self, value: model::Resume) -> Key {
        self.state.add(value)
    }
}

/// An object for representing application state.
#[derive(Debug)]
struct State {
    resumes: TempCache<model::Resume>,
}

impl State {
    fn new() -> Self {
        Self {
            resumes: TempCache::new(),
        }
    }
}

/// Add a new resume to a State object--defers to underlying cache.
impl Add<model::Resume> for State {
    fn add(&mut self, value: model::Resume) -> Key {
        self.resumes.add(value)
    }
}

/// A db cache separating values found from the db (cache) from values that have yet to be saved
/// to the db (temp).
#[derive(Debug)]
struct TempCache<V> {
    next_tmp_key: i64,
    cache: HashMap<Key, V>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    Tmp(i64),
    Db(i64),
}

impl<V> TempCache<V> {
    /// Inits an empty cache w/ no values in either HashMap.
    fn new() -> Self {
        Self {
            next_tmp_key: 0,
            cache: HashMap::new(),
        }
    }
}

impl<V> Add<V> for TempCache<V> {
    /// Add a new value to the cache. Places new items in temp, allowing them to be added to the
    /// database & moved to cache with their actual id value on save.
    fn add(&mut self, value: V) -> Key {
        let key = Key::Tmp(self.next_tmp_key);
        self.cache.insert(key, value);
        self.next_tmp_key += 1;
        key
    }
}

#[async_trait]
impl<V: Send> Get<V> for TempCache<V> {
    /// Extract a value with a matching id from the cache, or the underlying data store if not
    /// found in cache (updating the cache when found), or None if no matching value is found.
    async fn get(&mut self, id: Key) -> Option<&V> {
        // first we check if the requested id is in the cache
        if let Some(value) = self.cache.get(&id) {
            return Some(value);
        } else {
            // TODO: actually get this from the db instead
            None
        }
        // finally, if not found in either, we check the db & load it into the cache if found
    }
}

/// Add a value to an implementing structure, returning the values new id number.
pub trait Add<T> {
    fn add(&mut self, value: T) -> Key;
}

#[async_trait]
/// Allow values to be borrowed by their id.
pub trait Get<V: Send> {
    /// Find a value by it's id & borrow it, if it exists.
    async fn get(&mut self, id: Key) -> Option<&V>;
}
