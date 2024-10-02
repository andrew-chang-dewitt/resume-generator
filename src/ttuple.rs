//! A tiny two-tuple implementation
//
// TODO:
// - [x] impl peek
// - [x] impl + operator
// - [ ] impl get_all
// Maybe?:
// - [ ] impl pluck
// - [ ] impl pluck_all

use std::{any::Any, fmt::Debug, marker::PhantomData, ops::Add};

/// Core Ttuple list behaviors.
///
/// ## Examples
///
/// TODO:
/// - [ ] Add an item to a list
/// - [ ] Get the length of a list
/// - [ ] Iterate over a list?
pub trait HList: Sized + Debug + Eq {
    /// An HList knows it's length
    fn len(&self) -> usize;

    /// An HList can add an item to the collection
    fn prepend<H: Sized>(self, h: H) -> Ttuple<H, Self> {
        Ttuple(h, self)
    }
}

/// Nil marks the end of a list.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Nil();

impl HList for Nil {
    fn len(&self) -> usize {
        0
    }
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
    fn len(&self) -> usize {
        1 + <T as HList>::len(&self.1)
    }
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

#[cfg(test)]
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

#[cfg(test)]
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

trait GetSome<Select> {
    fn get_some(&self) -> Option<&Select>;
    fn get_some_mut(&mut self) -> Option<&mut Select>;
}

impl<Nonexistant> GetSome<Nonexistant> for Nil {
    fn get_some(&self) -> Option<&Nonexistant> {
        None
    }

    fn get_some_mut(&mut self) -> Option<&mut Nonexistant> {
        None
    }
}

#[cfg(test)]
#[test]
fn getting_anything_from_nil_gives_none() {
    let n = Nil();
    let g: Option<&i32> = n.get_some();

    if let Some(v) = g {
        unreachable!("got `{v:#?}` when should've gotten `None`")
    }

    assert_eq!(g, None)
}

impl<Select, Head, Tail> GetSome<Select> for Ttuple<Head, Tail>
where
    Select: 'static,
    Head: Any,
    Tail: HList + GetSome<Select>,
{
    fn get_some(&self) -> Option<&Select> {
        let any_head = &self.0 as &dyn Any;

        match any_head.downcast_ref::<Select>() {
            Some(selected) => Some(&selected),
            None => self.1.get_some(),
        }
    }

    fn get_some_mut(&mut self) -> Option<&mut Select> {
        let any_head = &mut self.0 as &mut dyn Any;

        match any_head.downcast_mut::<Select>() {
            Some(selected) => Some(selected),
            None => self.1.get_some_mut(),
        }
    }
}

#[cfg(test)]
#[test]
fn can_get_some_from_head() {
    let t = Ttuple(1i32, Nil());
    let g: Option<&i32> = t.get_some();

    if let Some(v) = g {
        assert_eq!(v, &1i32)
    } else {
        unreachable!("got `{g:#?}` when should've gotten `1i32`")
    }
}

#[cfg(test)]
#[test]
fn can_get_none_if_not_in_ttuple() {
    let t = Ttuple(1i32, Ttuple(true, Nil()));
    let g: Option<&&str> = t.get_some();

    if let Some(v) = g {
        unreachable!("got `{v:#?}` when should've gotten `None`")
    } else {
        assert_eq!(g, None)
    }
}

#[cfg(test)]
#[test]
fn get_some_can_be_used_to_check_if_ttuple_contains_type() {
    let t = Ttuple(1i32, Ttuple("str", Ttuple::new(false)));

    let yes: Option<&bool> = t.get_some();
    if let Some(_) = yes {
        assert!(true, "{t:#?} contains a boolean");
    };

    let no: Option<&String> = t.get_some();
    if let None = no {
        assert!(true, "{t:#?} does not contain a String");
    };

    let nil: Option<&Nil> = t.get_some();
    if let Some(_) = nil {
        assert!(true, "all Ttuples always contain Nil");
    };
}

// TODO:
// the below can't work because there's no good way to advance to the tail w/out recursing
// think we need to make a GetAll or Iterator trait that depends on GetSome and recursively calls
// GetSome on the tail--this'll need some way of tracking where we are still though--maybe using
// Here & There<I>? Not sure yet, this'll take some thought...
// #[cfg(test)]
// #[test]
// fn get_some_can_be_used_to_loop_over_all_of_type() {
//     let t = Ttuple(1i32, Ttuple("str", Ttuple(false, Ttuple::new(2i32))));
//
//     let mut sum = 0;
//     let mut keep_going = true;
//     let mut count = 0;
//
//     loop {
//         println!("loop iteration START");
//         println!("sum is {sum:#?}");
//         println!("count is {count:#?}");
//         let int = t.get_some();
//         println!("int is {int:#?}");
//         if let Some(i) = int {
//             println!("inside if let Some(int) w/ {i:#?}");
//             sum += i;
//         } else {
//             keep_going = false;
//         }
//         println!("loop iteration END");
//
//         count += 1;
//
//         if !keep_going || count > 10 {
//             println!("loop BREAK");
//             println!("sum is {sum:#?}");
//             println!("count is {count:#?}");
//             break;
//         }
//     }
//
//     assert_eq!(sum, 3);
// }

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
#[test]
fn nil_extends_nil() {
    let n1 = Nil();
    let n2 = Nil();

    assert_eq!(n1 + n2, Nil());
}

#[cfg(test)]
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

#[cfg(test)]
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

#[cfg(test)]
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
