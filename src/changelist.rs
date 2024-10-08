//! A `ChangeList` is simply a stack of state change operations.
//!
//! For any two [`state::State`]'s, there can be at least one `ChangeList` that executed in order,
//! will transform one [`state::State`] into the other.
//! This means a `ChangeList` is a series of atomic state change operations to which items are
//! typically added by prepending to the head & removed by popping the most recently added value.
#[derive(Debug)]
pub struct ChangeList<SomeState: Clone, Index>(Vec<Box<dyn ApplyTo<SomeState, Index>>>);

impl<SomeState: Clone, Index> ChangeList<SomeState, Index> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push<C>(&mut self, change: C)
    where
        C: ApplyTo<SomeState, Index> + 'static,
    {
        self.0.push(Box::new(change))
    }

    fn pop_into(&mut self, other: &mut ChangeList<SomeState, Index>) {
        if let Some(change) = self.0.pop() {
            other.0.push(change)
        }
    }

    fn apply_all(&self, state: &mut SomeState) -> Vec<Index> {
        self.apply_all_from(state, 0)
    }

    fn apply_all_from(&self, state: &mut SomeState, start: usize) -> Vec<Index> {
        let to_apply = &self.0[start..];
        let mut res = Vec::new();
        for change in to_apply {
            res.push((**change).apply_to(state));
        }

        res
    }
}

pub trait ApplyTo<SomeState, Index>: std::fmt::Debug {
    fn apply_to(&self, state: &mut SomeState) -> Index;
}

pub trait Apply<Change, Index> {
    fn apply(&mut self, change: &Change) -> Index;
}

impl<S, I, C> ApplyTo<S, I> for C
where
    S: Apply<C, I>,
    C: std::fmt::Debug,
{
    fn apply_to(&self, state: &mut S) -> I {
        S::apply(state, self)
    }
}

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

    impl ApplyTo<Vec<String>, usize> for String {
        fn apply_to(&self, state: &mut Vec<String>) -> usize {
            state.push(self.clone());
            state.len() - 1
        }
    }

    #[test]
    fn simple_add_one_to_empty() {
        // starting with an empty State
        let start: Vec<String> = Vec::new();
        // and a list of 1 Change: adding a new item to the State
        let mut changes: ChangeList<Vec<String>, usize> = ChangeList::new();
        changes.push("added".to_string());
        // applying those changes should result in a State that contains the 1 item
        let mut end = start.clone();
        changes.apply_all(&mut end);

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
        let mut changes: ChangeList<Vec<String>, usize> = ChangeList::new();
        changes.push("new!".to_string());
        // applying those changes should result in a State that contains existing data, plus the 1 item
        let mut end = start.clone();
        changes.apply_all(&mut end);

        let expected = vec!["existing", "new!"];
        assert_eq!(
            end, expected,
            "applying {changes:?} to {start:?} should create new state of {expected:?}, not {end:?}"
        );
    }

    impl ApplyTo<Vec<String>, usize> for (usize, String) {
        fn apply_to(&self, state: &mut Vec<String>) -> usize {
            state[self.0] = self.1.clone();
            self.0
        }
    }

    #[test]
    fn simple_alter_existing_item() {
        // starting with a state containing some data already
        let start: Vec<String> = vec!["existing".into()];
        // and a list of 1 Change: adding a new item to the State
        let mut changes: ChangeList<Vec<String>, usize> = ChangeList::new();
        changes.push((0, "altered!".to_string()));
        // applying those changes should result in a State that contains existing data, plus the 1 item
        let mut end = start.clone();
        changes.apply_all(&mut end);

        let expected = vec!["altered!"];
        assert_eq!(
            end, expected,
            "applying {changes:?} to {start:?} should create new state of {expected:?}, not {end:?}"
        );
    }
}
