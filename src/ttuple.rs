//! A tiny two-tuple implementation
//
// TODO:
// - [x] impl peek
// - [x] impl + operator
// - [ ] impl get_all
// Maybe?:
// - [ ] impl pluck
// - [ ] impl pluck_all

use std::{
    any::{Any, TypeId},
    fmt::Debug,
    marker::PhantomData,
    ops::Add,
};

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

/// All lists are built of nested Ttuple instances
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

/// Borrow the first item from a two-tuple matching a given type
trait Get<Select, Index> {
    fn get(&self) -> &Select;
    fn get_mut(&mut self) -> &mut Select;
}

impl<FromHead: Sized, Tail> Get<FromHead, Here> for Ttuple<FromHead, Tail> {
    fn get(&self) -> &FromHead {
        &self.0
    }

    fn get_mut(&mut self) -> &mut FromHead {
        &mut self.0
    }
}

impl<Head, Tail, FromTail, Index> Get<FromTail, There<Index>> for Ttuple<Head, Tail>
where
    Head: Sized,
    Tail: Get<FromTail, Index>,
{
    fn get(&self) -> &FromTail {
        self.1.get()
    }

    fn get_mut(&mut self) -> &mut FromTail {
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

trait ContainsA {
    fn contains_a(self, select_type: &dyn Any) -> bool;
}

impl ContainsA for Nil {
    fn contains_a(self, select_type: &dyn Any) -> bool {
        let t = select_type.type_id();
        (TypeId::of::<Nil>() == t) || (TypeId::of::<()>() == t)
    }
}

#[test]
fn can_check_if_nil_contains_something() {
    let t = Nil();

    let boolean = t.contains_a(&true);
    assert!(!boolean, "Nil can't contain a boolean");

    let nil = t.contains_a(&Nil());
    assert!(nil, "however, Nil can contain itself");

    let nothing = t.contains_a(&());
    assert!(nothing, "and () can be a shorthand for Nil");
}

impl<InHead: 'static, InTail> ContainsA for Ttuple<InHead, InTail>
where
    InTail: ContainsA,
{
    fn contains_a(self, select_type: &dyn Any) -> bool {
        (TypeId::of::<InHead>() == select_type.type_id()) || self.1.contains_a(select_type)
    }
}

#[test]
fn can_check_if_contains_a_type() {
    let t = Ttuple(1i32, Ttuple("str", Ttuple::new(false)));

    let yes = t.contains_a(&true);
    assert!(yes, "{t:#?} contains a boolean");

    let no = t.contains_a(&String::new());
    assert!(!no, "{t:#?} does not contain a String");

    let nil = t.contains_a(&Nil());
    assert!(nil, "all Ttuples always contain Nil");
}

#[test]
fn contains_can_be_a_typeguard() {
    let t = Ttuple::new(1i32);

    if t.contains_a(&0i32) {
        assert!(true, "{t:#?} contains an i32, this should be evaluated");
    }

    match t.contains_a(&"test") {
        true => {
            assert!(
                false,
                "{t:#?} does not contain a &str, this should not be evaluated"
            );
            // so this should still compile?
            let _: &&str = t.get();
        }
        false => assert!(true, "{t:#? does not contain a &str}"),
    }
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

/// Ttuples can be concatenated using Add operator
impl<RHS> Add<RHS> for Nil
where
    RHS: HList,
{
    type Output = RHS;

    fn add(self, rhs: RHS) -> Self::Output {
        rhs
    }
}

impl<H, T, RHS> Add<RHS> for Ttuple<H, T>
where
    T: Add<RHS>,
    RHS: HList,
{
    type Output = Ttuple<H, <T as Add<RHS>>::Output>;

    fn add(self, rhs: RHS) -> Self::Output {
        Ttuple(self.0, self.1 + rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_borrow_items_by_type() {
        let t = Ttuple(
            2i32,
            Ttuple("first", Ttuple(vec![1i32, 2, 3], Ttuple::new(Some(false)))),
        );
        let two: &i32 = t.get();
        let one: &&str = t.get();
        assert_eq!(two, &2);
        assert_eq!(one, &"first");
    }

    #[test]
    fn can_mutably_borrow_items_by_type() {
        let tmp = Vec::from([1i32, 2, 3]);
        let mut t = Ttuple(2i32, Ttuple("first", Ttuple::new(tmp)));
        *t.get_mut() = 3;
        *t.get_mut() = "updated";
        let v: &mut Vec<i32> = t.get_mut();
        for i in 0..v.len() {
            v[i] *= 2
        }
        assert_eq!(
            t,
            Ttuple(3i32, Ttuple("updated", Ttuple::new(vec![2i32, 4, 6])))
        );
    }

    #[test]
    fn can_pop_head() {
        let tmp = Vec::from([1i32, 2, 3]);
        let tt = Ttuple(2i32, Ttuple("first", Ttuple::new(tmp)));
        let (h, t) = tt.pop();
        assert_eq!(h, 2);
        assert_eq!(t, Ttuple("first", Ttuple::new(vec![1i32, 2, 3])));
        let (i, u) = t.pop();
        assert_eq!(i, "first");
        assert_eq!(u, Ttuple::new(vec![1i32, 2, 3]));
    }

    #[test]
    fn can_peek_head() {
        let tmp = Vec::from([1i32, 2, 3]);
        let tt = Ttuple(2i32, Ttuple("first", Ttuple::new(tmp)));
        let h = tt.peek();
        assert_eq!(h, &2);
        assert_eq!(
            tt,
            Ttuple(2i32, Ttuple("first", Ttuple::new(vec![1i32, 2, 3])))
        );
    }

    #[test]
    fn nil_extends_nil() {
        let n1 = Nil();
        let n2 = Nil();

        assert_eq!(n1 + n2, Nil());
    }

    #[test]
    fn ttuple_extends_nil() {
        let n1 = Nil();
        let t1 = Ttuple(
            1i32,
            Ttuple("first", Ttuple(vec![1i32, 2, 3], Ttuple::new(Some(false)))),
        );

        assert_eq!(
            n1 + t1,
            Ttuple(
                1i32,
                Ttuple("first", Ttuple(vec![1i32, 2, 3], Ttuple::new(Some(false)))),
            )
        );
    }

    #[test]
    fn nil_extends_ttuple() {
        let n1 = Nil();
        let t1 = Ttuple(
            1i32,
            Ttuple("first", Ttuple(vec![1i32, 2, 3], Ttuple::new(Some(false)))),
        );

        assert_eq!(
            t1 + n1,
            Ttuple(
                1i32,
                Ttuple("first", Ttuple(vec![1i32, 2, 3], Ttuple::new(Some(false)))),
            )
        );
    }

    #[test]
    fn ttuple_extends_ttuple() {
        let t1 = Ttuple(
            1i32,
            Ttuple("first", Ttuple(vec![1i32, 2, 3], Ttuple::new(Some(false)))),
        );
        let t2 = Ttuple(2i32, Ttuple::new("second"));
        let t3 = t1 + t2;

        assert_eq!(
            t3,
            Ttuple(
                1i32,
                Ttuple(
                    "first",
                    Ttuple(
                        vec![1i32, 2, 3],
                        Ttuple(Some(false), Ttuple(2i32, Ttuple::new("second")))
                    )
                )
            )
        );
    }
}
