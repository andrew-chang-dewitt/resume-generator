//! A collection data structure for caching application data in memory.
//
// TODO:
// - [x] impl Get from Store deferred down to Tempcaches on State
// - [ ] impl a save method on Store
// - [ ] impl db logic on Get
// - [ ] some sort of Update or Modify trait
// - [x] track Changes in vec on Store
// - [ ] ? undo/redo Changes
use sqlx::SqlitePool;

use crate::{
    changelist::{Apply, ChangeList},
    model,
    state::{AddNew, AppState, Key},
};

/// A data store, containing application state & handling db updates.
#[derive(Debug)]
pub struct Store {
    pool: SqlitePool,
    // TODO: track two State objects: initial state (from db) & current state (initial modified by
    // changes)--allowing us to only apply changes from initial state to db on save
    // this maybe leads to also maintaining a vec of Changes, where if one were to apply
    // each Change in order to the initial state, they would arrive at the current state--this
    // might give "free" undo functionality if we can also can implement some sort of Inverse
    // for every Change
    initial: AppState,
    current: AppState,
    // for now, assume that current is always up to date from all changes
    // enforce this by: always applying a change when pushing it to changes
    //                  and always applying the inverse change when popping it to undone
    changes: ChangeList<AppState, Key>,
    // undone: ChangeList<AppState, Key>,
}

impl Store {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool,
            initial: AppState::new(),
            current: AppState::new(),
            changes: ChangeList::new(),
            // undone: ChangeList::new(),
        }
    }
}

/// Add resumes to store, deferring to state.
impl AddNew<model::Resume, Key> for Store {
    fn add_new(&mut self, value: model::Resume) -> Key {
        let new_key = self.current.apply(&value);
        self.changes.push(value);

        new_key
    }
}

impl Apply<model::Resume, Key> for AppState {
    fn apply(&mut self, change: &model::Resume) -> Key {
        self.add_new(change.clone())
    }
}
