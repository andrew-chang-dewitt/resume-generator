//! A tiny two-tuple implementation
//
// TODO:
// - [ ] impl peek
// - [ ] impl into iter, using peek

use std::{fmt::Debug, marker::PhantomData};

/// Create an empty two-tuple
///
/// ## Example
///
/// ```
/// todo!()
/// ```
pub fn ttuple_empty() -> Nil {
    Nil()
}

/// Create a two-tuple from a single item
///
/// ## Example
///
/// ```
/// todo!()
/// ```
pub fn ttuple_one<H: Sized + Debug + Eq>(initial: H) -> Ttuple<H, Nil> {
    Ttuple::new(initial)
}

/// Create a two tuple from a new head & an existing two-tuple
///
/// ## Example
///
/// ```
/// todo!()
/// ```
pub fn ttuple<H: Sized, T: HList>(head: H, tail: T) -> Ttuple<H, T> {
    Ttuple(head, tail)
}

/// Core Ttuple behaviors.
pub trait HList: Sized + Debug + Eq {
    /// Return length of ttuple
    const LEN: usize;

    /// Get the length of the ttuple
    fn len(&self) -> usize {
        Self::LEN
    }

    /// Add an item to the collection
    fn prepend<H: Sized>(self, h: H) -> Ttuple<H, Self> {
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
struct Ttuple<H: Sized, T = Nil>(H, T);

impl<H: Sized> Ttuple<H> {
    fn new(head: H) -> Self {
        Ttuple(head, Nil())
    }
}

impl<H: Sized + Debug + Eq, T: HList> HList for Ttuple<H, T> {
    const LEN: usize = 1 + <T as HList>::LEN;
}

// TODO: maybe come back to this
// impl<H, T: HList> From<(H, T)> for Ttuple<H, T> {
//     fn from(value: (H, T)) -> Self {
//         value.1.into().prepend(value.0)
//     }
// }

/// Borrow the first item from a two-tuple matching a given type
trait Get<T, I> {
    fn get(&self) -> &T;
    fn get_mut(&mut self) -> &mut T;
}

impl<H: Sized, Tail> Get<H, Here> for Ttuple<H, Tail> {
    fn get(&self) -> &H {
        &self.0
    }

    fn get_mut(&mut self) -> &mut H {
        &mut self.0
    }
}

impl<H, T, F, I> Get<F, There<I>> for Ttuple<H, T>
where
    H: Sized,
    T: Get<F, I>,
{
    fn get(&self) -> &F {
        self.1.get()
    }

    fn get_mut(&mut self) -> &mut F {
        self.1.get_mut()
    }
}

/// Type for matching the index when the head 
/// is the type requested by Getter::get
struct Here {
    _priv: (),
}

/// Type for matching the index when the type 
// requested by Getter::get is not in the head
struct There<T> {
    _priv: PhantomData<T>,
}

/// Destructively remove first item in list, returning it along with a new Ttuple made from the
/// tail of the list
trait Pop<H, T> {
    fn pop(self) -> (H, T);
}

impl<H, T: HList> Pop<H, T> for Ttuple<H, T> {
    fn pop(self) -> (H, T) {
        (self.0, self.1)
    }
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
        let tmp = Vec::from([1i32, 2, 3]);
        let t = ttuple(2i32, ttuple(("first"), ttuple_one(tmp)));
        let two: &i32 = t.get();
        let one: &&str = t.get();
        assert_eq!(two, &2);
        assert_eq!(one, &"first");
    }

    #[test]
    fn test_can_mutably_borrow_items_by_type() {
        let mut tmp = Vec::from([1i32, 2, 3]);
        let mut t = ttuple(2i32, ttuple("first", ttuple_one(tmp)));
        *t.get_mut() = 3;
        *t.get_mut() = "updated";
        let mut v: &mut Vec<i32> = t.get_mut();
        for i in 0..v.len() {
            v[i] *= 2
        }
        assert_eq!(
            t,
            ttuple(3i32, ttuple("updated", ttuple_one(vec![2i32, 4, 6])))
        );
    }

    #[test]
    fn test_can_pop_head() {
        let tmp = Vec::from([1i32, 2, 3]);
        let tt = ttuple(2i32, ttuple("first", ttuple_one(tmp)));
        let (h, t) = tt.pop();
        assert_eq!(h, 2);
        assert_eq!(t, ttuple("first", ttuple_one(vec![1i32, 2, 3])));
        let (i, u) = t.pop();
        assert_eq!(i, "first");
        assert_eq!(u, ttuple_one(vec![1i32, 2, 3]));
    }
}
