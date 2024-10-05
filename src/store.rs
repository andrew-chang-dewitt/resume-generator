//! A collection data structure for caching application data in memory.
//
// TODO:
// - [ ] impl Get from Store deferred down to Tempcaches on State
// - [ ] impl a save method on Store
// - [ ] impl db logic on Get
// - [ ] some sort of Update or Modify trait
// - [ ] track Changes in vec on Store
use std::collections::HashMap;

use async_trait::async_trait;
use sqlx::SqlitePool;

use crate::model::{self, Model, ModelKey};

/// A data store, containing application state & handling db updates.
#[derive(Debug)]
pub struct Store {
    pool: SqlitePool,
    // TODO: track two state objects: initial state (from db) & current state initial modified by
    // changes--allowing us to only apply changes from initial state to db on save
    // this maybe leads to also maintaining a vec of changes, where if one were to apply
    // each change in order to the initial state, they would arrive at the current state--this
    // might give "free" info functionality if desired
    state: State,
}

impl Store {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool,
            state: State::new(),
        }
    }

    // Add a new instance of any Model to the store
    pub fn add(&mut self, value: Model) -> ModelKey<Key> {
        self.state.add(value)
    }

    async fn get(&self, id: ModelKey<Key>) -> Option<&Model> {
        self.state.get(id).await
    }
    async fn get_mut(&mut self, id: ModelKey<Key>) -> Option<&mut Model> {
        self.state.get_mut(id).await
    }
}

/// Add a value to an implementing structure, returning the values new id number.
pub trait Add<T> {
    fn add(&mut self, value: T) -> Key;
}

#[async_trait]
/// Allow values to be borrowed by their id.
pub trait Get<V: Send + Sync> {
    /// Find a value by it's id & borrow it, if it exists.
    async fn get(&self, id: Key) -> Option<&V>;
    async fn get_mut(&mut self, id: Key) -> Option<&mut V>;
}

/// An object for representing application state.
///
/// TODO: Should this be a hashmap of strings to tempcaches? This would allow traversing the state
/// types when saving changes or loading a lot of data into the cache at once (i.e. warming the
/// cache) while still allowing access to the state by key. Maybe the key & associated type could
/// be a user-defined enum even, allowing to restrict the State to pre-determined types.
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

    fn add(&mut self, value: Model) -> ModelKey<Key> {
        match value {
            Model::Resume(r) => self.resumes.add(r),
            _ => unimplemented!(),
        }
    }

    fn get(&self, id: ModelKey<Key>) -> Model {
        todo!()
    }
}

#[async_trait]
impl Get<model::Resume> for State {
    async fn get(&self, id: Key) -> Option<&model::Resume> {
        self.resumes.get(id).await
    }
    async fn get_mut(&mut self, id: Key) -> Option<&mut model::Resume> {
        self.resumes.get_mut(id).await
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
impl<V: Send + Sync> Get<V> for TempCache<V> {
    /// Extract a value with a matching id from the cache, or the underlying data store if not
    /// found in cache (updating the cache when found), or None if no matching value is found.
    async fn get(&self, id: Key) -> Option<&V> {
        // first we check if the requested id is in the cache
        if let Some(value) = self.cache.get(&id) {
            return Some(value);
        } else {
            // TODO: actually get this from the db instead
            None
        }
        // finally, if not found in either, we check the db & load it into the cache if found
    }

    async fn get_mut(&mut self, id: Key) -> Option<&mut V> {
        // first we check if the requested id is in the cache
        if let Some(value) = self.cache.get_mut(&id) {
            return Some(value);
        } else {
            // TODO: actually get this from the db instead
            None
        }
        // finally, if not found in either, we check the db & load it into the cache if found
    }
}
