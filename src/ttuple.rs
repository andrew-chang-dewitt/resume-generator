//! A tiny two-tuple implementation

use std::fmt::Debug;

/// Core Ttuple behaviors.
pub trait HList: Sized + Debug + Eq {
    /// Return length of ttuple
    const LEN: usize;

    /// Get the length of the ttuple
    fn len(&self) -> usize {
        Self::LEN
    }

    /// Add an item to the collection
    fn prepend<H>(self, h: H) -> Ttuple<H, Self> {
        Ttuple(h, self)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Nil();

impl HList for Nil {
    const LEN: usize = 0;
}

impl From<()> for Nil {
    fn from(_: ()) -> Self {
        Nil()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Ttuple<H, T: HList = Nil>(H, T);

impl<H> Ttuple<H> {
    fn new(head: H) -> Self {
        Ttuple(head, Nil())
    }
}

impl<H: Debug + Eq, T: HList> HList for Ttuple<H, T> {
    const LEN: usize = 1 + <T as HList>::LEN;
}

// TODO: maybe come back to this
// impl<H, T: HList> From<(H, T)> for Ttuple<H, T> {
//     fn from(value: (H, T)) -> Self {
//         value.1.into().prepend(value.0)
//     }
// }

pub fn ttuple_empty() -> Nil {
    Nil()
}

pub fn ttuple_one<I: Debug + Eq>(initial: I) -> Ttuple<I, Nil> {
    Ttuple::new(initial)
}

pub fn ttuple<H, T: HList>(head: H, tail: T) -> Ttuple<H, T> {
    Ttuple(head, tail)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_ttuple() {
        let t = ttuple_empty();
        assert_eq!(t.len(), 0);
    }

    #[test]
    fn test_two_item_ttuple() {
        let t = ttuple_one("item");
        println!("{t:#?}"); // => ("item", ())
        assert_eq!(t.len(), 1);
        assert_eq!(t.0, "item");
        assert_eq!(t.1, ().into());
        assert_eq!(t.1.len(), 0);
    }

    #[test]
    fn test_can_add_new_items() {
        let t = ttuple_empty();
        let f = t.prepend("first");
        assert_eq!(f.len(), 1);
        assert_eq!(f.0, "first");
        let s = f.prepend(2);
        assert_eq!(s.len(), 2);
        assert_eq!(s.0, 2, "first element of {f:#?} should be 2");
        assert_eq!(
            s.1,
            ttuple("first", Nil()),
            "tail of {f:#?} should be ('first', ())"
        );
    }

    #[test]
    fn test_can_borrow_items_by_type() {
        todo!()
    }
}
