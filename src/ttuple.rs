//! A tiny two-tuple implementation
//
// TODO:
// - [ ] impl peek
// - [ ] impl into iter, using peek

use std::{fmt::Debug, marker::PhantomData};

/// Core Ttuple list behaviors.
///
/// ## Examples
///
/// TODO:
/// - [ ] Add an item to a list
/// - [ ] Get the length of a list
/// - [ ] Iterate over a list?
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

/// Nil marks the end of a list.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Nil();

impl HList for Nil {
    const LEN: usize = 0;
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Ttuple<H: Sized, T = Nil>(H, T);

impl<H: Sized> Ttuple<H> {
    fn new(head: H) -> Self {
        Ttuple(head, Nil())
    }
}

impl<H: Sized + Debug + Eq, T: HList> HList for Ttuple<H, T> {
    const LEN: usize = 1 + <T as HList>::LEN;
}

// impl From<()> for Nil {
//     fn from(_: ()) -> Self {
//         Nil()
//     }
// }

impl<H> From<(H, ())> for Ttuple<H> {
    fn from((h, _): (H, ())) -> Self {
        Ttuple(h, Nil())
    }
}

impl<Head, TailFrom, TailInto> From<(Head, TailFrom)> for Ttuple<Head, TailInto>
where
    TailInto: HList + From<TailFrom>,
{
    fn from((h, t): (Head, TailFrom)) -> Self {
        let tail: TailInto = t.into();
        
        tail.prepend(h)
    }
}

#[test]
fn create_ttuple_from_tuple() {
    // let n = ();
    // assert_eq!(Nil::from(n), Nil());

    let one = (1, ());
    assert_eq!(Ttuple::from(one), Ttuple(1, Nil()));

    let t = (2i32, ("first", (vec![1i32, 2, 3], (Some(false), ()))));
    let tt = Ttuple::from(t);
    assert_eq!(tt.0, 2i32,);
    assert_eq!((tt.1).0, "first",);
    assert_eq!(((tt.1).1).0, vec![1i32, 2, 3],);
    assert_eq!((((tt.1).1).1).0, Some(false),);
    assert_eq!(((((tt.1).1).1).1).0, Nil(),);
}

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

/// Look at the first item on the list by borrowing it without altering the list
trait Peek<H> {
    fn peek(&self) -> &H;
}

impl<H, T> Peek<H> for Ttuple<H, T> {
    fn peek(&self) -> &H {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn can_borrow_items_by_type() {
    //     let t = (2i32, ("first", (vec![1i32, 2, 3], (Some(false), ()))));
    //     let tt = Ttuple::from(t);
    //     let two: &i32 = tt.get();
    //     let one: &&str = tt.get();
    //     assert_eq!(two, &2);
    //     assert_eq!(one, &"first");
    // }

    // #[test]
    // fn can_mutably_borrow_items_by_type() {
    //     let mut tmp = Vec::from([1i32, 2, 3]);
    //     let mut t = ttuple(2i32, ttuple("first", ttuple_one(tmp)));
    //     *t.get_mut() = 3;
    //     *t.get_mut() = "updated";
    //     let v: &mut Vec<i32> = t.get_mut();
    //     for i in 0..v.len() {
    //         v[i] *= 2
    //     }
    //     assert_eq!(
    //         t,
    //         ttuple(3i32, ttuple("updated", ttuple_one(vec![2i32, 4, 6])))
    //     );
    // }

    // #[test]
    // fn can_pop_head() {
    //     let tmp = Vec::from([1i32, 2, 3]);
    //     let tt = ttuple(2i32, ttuple("first", ttuple_one(tmp)));
    //     let (h, t) = tt.pop();
    //     assert_eq!(h, 2);
    //     assert_eq!(t, ttuple("first", ttuple_one(vec![1i32, 2, 3])));
    //     let (i, u) = t.pop();
    //     assert_eq!(i, "first");
    //     assert_eq!(u, ttuple_one(vec![1i32, 2, 3]));
    // }

    // #[test]
    // fn can_peek_head() {
    //     let tmp = Vec::from([1i32, 2, 3]);
    //     let tt = ttuple(2i32, ttuple("first", ttuple_one(tmp)));
    //     let h = tt.peek();
    //     assert_eq!(h, &2);
    //     assert_eq!(
    //         tt,
    //         ttuple(2i32, ttuple("first", ttuple_one(vec![1i32, 2, 3])))
    //     );
    // }
}
