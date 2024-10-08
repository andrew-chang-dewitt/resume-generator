//! A collection data structure for caching application data in memory.
//
// TODO:
// - [ ] impl Get from Store deferred down to Tempcaches on State
// - [ ] impl a save method on Store
// - [ ] impl db logic on Get
// - [ ] some sort of Update or Modify trait
// - [ ] track Changes in vec on Store
use sqlx::SqlitePool;

use crate::{
    model,
    state::{AddNew, AppState, Key},
};

/// A data store, containing application state & handling db updates.
#[derive(Debug)]
pub struct Store {
    pool: SqlitePool,
    // TODO: track two state objects: initial state (from db) & current state initial modified by
    // changes--allowing us to only apply changes from initial state to db on save
    // this maybe leads to also maintaining a vec of changes, where if one were to apply
    // each change in order to the initial state, they would arrive at the current state--this
    // might give "free" info functionality if desired
    state: AppState,
}

impl Store {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool,
            state: AppState::new(),
        }
    }
}

/// Add resumes to store, deferring to state.
impl AddNew<model::Resume, Key> for Store {
    fn add_new(&mut self, value: model::Resume) -> Key {
        self.state.add_new(value)
    }
}
