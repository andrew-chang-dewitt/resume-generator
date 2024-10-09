use std::{fmt::Display, marker::PhantomData};

use crate::store::Store;

struct AppRouter<RouterState> {
    state: RouterState,
    store: Store,
}

struct Idle<L: Location> {
    _at: PhantomData<L>,
}

impl<L: Location> Idle<L> {
    pub fn new() -> Self {
        Self { _at: PhantomData }
    }
}

struct Navigating<FromL: Location, ToL: Location> {
    _from: PhantomData<FromL>,
    _to: PhantomData<ToL>,
}

impl<FromL: Location, ToL: Location> Navigating<FromL, ToL> {
    pub fn new() -> Self {
        Self {
            _from: PhantomData,
            _to: PhantomData,
        }
    }
}

struct Exiting<L: Location> {
    _referrer: PhantomData<L>,
}

impl<L: Location> Exiting<L> {
    pub fn new() -> Self {
        Self {
            _referrer: PhantomData,
        }
    }
}

// struct Canceling<State, L: Location> {
//     canceled: State,
//     _revert_to: PhantomData<L>,
// }

// struct Saving<RefL: Location, NxtL: Location = RefL> {
//     _referrer: PhantomData<RefL>,
//     _next: PhantomData<NxtL>,
// }

// struct Error<State, RefL: Location, RevL: Location> {
//     source: State,
//     _referrer: PhantomData<RefL>,
//     _revert_to: PhantomData<RevL>,
// }

impl<L: Location> AppRouter<Idle<L>> {
    pub fn new(store: Store) -> Self {
        Self {
            state: Idle::new(),
            store,
        }
    }
}

impl<FromL, ToL> From<AppRouter<Idle<FromL>>> for AppRouter<Navigating<FromL, ToL>>
where
    FromL: Location,
    ToL: Location,
{
    fn from(value: AppRouter<Idle<FromL>>) -> Self {
        Self {
            state: Navigating::new(),
            store: value.store,
        }
    }
}

impl<FromL, ToL> From<AppRouter<Navigating<FromL, ToL>>> for AppRouter<Idle<ToL>>
where
    FromL: Location,
    ToL: Location,
{
    fn from(value: AppRouter<Navigating<FromL, ToL>>) -> Self {
        Self {
            state: Idle::new(),
            store: value.store,
        }
    }
}

impl<L: Location> From<AppRouter<Idle<L>>> for AppRouter<Exiting<L>> {
    fn from(value: AppRouter<Idle<L>>) -> Self {
        Self {
            state: Exiting::new(),
            store: value.store,
        }
    }
}

trait Location {
    const PATH: &'static str;

    fn to_string(&self) -> String {
        Self::PATH.into()
    }
}
