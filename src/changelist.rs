//! A `ChangeList` is simply a stack of state change operations.
//!
//! For any two [`state::State`]'s, there can be at least one `ChangeList` that executed in order,
//! will transform one [`state::State`] into the other.
//! This means a `ChangeList` is a series of atomic state change operations to which items are
//! typically added by prepending to the head & removed by popping the most recently added value.
#[derive(Debug)]
pub struct ChangeList<SomeState: Clone>(Vec<Box<dyn ApplyTo<SomeState>>>);

impl<SomeState: Clone> ChangeList<SomeState> {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn push<C>(&mut self, change: C)
    where
        C: ApplyTo<SomeState> + 'static,
    {
        self.0.push(Box::new(change))
    }

    fn pop_into(&mut self, other: &mut ChangeList<SomeState>) {
        if let Some(change) = self.0.pop() {
            other.0.push(change)
        }
    }

    fn apply_all(&self, state: &SomeState) -> SomeState {
        self.0.iter().fold(state.clone(), |s, c| (**c).apply_to(&s))
    }
}

pub trait ApplyTo<SomeState>: std::fmt::Debug {
    fn apply_to(&self, state: &SomeState) -> SomeState;
}

pub trait FromApply<C> {
    fn from_apply(before: &Self, change: &C) -> Self;
}

impl<S, C> ApplyTo<S> for C
where
    S: FromApply<C>,
    C: std::fmt::Debug,
{
    fn apply_to(&self, state: &S) -> S {
        S::from_apply(state, self)
    }
}

// impl<SomeState: ApplyChange + std::fmt::Debug> ApplyTo<SomeState> for ChangeList<SomeState> {
//     fn apply_to(&self, state: &SomeState) -> SomeState {
//         self.changes.iter().fold(state, |s, c| s.apply(c))
//     }
// }

#[cfg(test)]
mod test_add_one {
    use async_trait::async_trait;

    use crate::state::{AddNew, Get};

    use super::*;

    impl AddNew<String, usize> for Vec<String> {
        fn add_new(&mut self, value: String) -> usize {
            self.push(value);
            self.len() - 1
        }
    }

    #[async_trait]
    impl Get<String, usize> for Vec<String> {
        async fn get(&self, key: &usize) -> Option<&String> {
            if *key < self.len() {
                Some(&self[*key])
            } else {
                None
            }
        }
    }

    impl ApplyTo<Vec<String>> for String {
        fn apply_to(&self, state: &Vec<String>) -> Vec<String> {
            let mut res = state.clone();
            res.push(self.clone());
            res
        }
    }

    #[test]
    fn simple_add_one_to_empty() {
        // starting with an empty State
        let start: Vec<String> = Vec::new();
        // and a list of 1 Change: adding a new item to the State
        let mut changes: ChangeList<Vec<String>> = ChangeList::new();
        changes.push("added".to_string());
        // applying those changes should result in a State that contains the 1 item
        let end = changes.apply_all(&start);

        let expected = vec!["added"];
        assert_eq!(
            end, expected,
            "applying {changes:?} to {start:?} should create new state of {expected:?}, not {end:?}"
        );
    }

    #[test]
    fn simple_add_one_to_existing() {
        // starting with a state containing some data already
        let start: Vec<String> = vec!["existing".into()];
        // and a list of 1 Change: adding a new item to the State
        let mut changes: ChangeList<Vec<String>> = ChangeList::new();
        changes.push("new!".to_string());
        // applying those changes should result in a State that contains existing data, plus the 1 item
        let end = changes.apply_all(&start);

        let expected = vec!["existing", "new!"];
        assert_eq!(
            end, expected,
            "applying {changes:?} to {start:?} should create new state of {expected:?}, not {end:?}"
        );
    }

    impl ApplyTo<Vec<String>> for (usize, String) {
        fn apply_to(&self, state: &Vec<String>) -> Vec<String> {
            let mut res = state.clone();
            res[self.0] = self.1.clone();
            res
        }
    }

    #[test]
    fn simple_alter_existing_item() {
        // starting with a state containing some data already
        let start: Vec<String> = vec!["existing".into()];
        // and a list of 1 Change: adding a new item to the State
        let mut changes: ChangeList<Vec<String>> = ChangeList::new();
        changes.push((0, "altered!".to_string()));
        // applying those changes should result in a State that contains existing data, plus the 1 item
        let end = changes.apply_all(&start);

        let expected = vec!["altered!"];
        assert_eq!(
            end, expected,
            "applying {changes:?} to {start:?} should create new state of {expected:?}, not {end:?}"
        );
    }
}
