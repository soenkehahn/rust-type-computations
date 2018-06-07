#![recursion_limit = "8"]
use std::marker::PhantomData;

fn main() {
    // let () = <() as SKI<Seq<I, Seq<I, Empty>>>>::reduce(PhantomData);
    // let () = <() as SKI<Seq<K, Seq<I, Seq<I, Empty>>>>>::reduce(PhantomData);
    // let () = <() as SKI<Seq<K, Seq<I, Empty>>>>::reduce(PhantomData);
    // let () = <() as SKI<Seq<K, Empty>>>::reduce(PhantomData);
    let () = <() as SKI<Seq<S, Seq<I, Seq<I, Seq<I, Empty>>>>>>::reduce(PhantomData);
}

// * symbols

struct X;
struct Y;
struct Z;

struct S;
struct K;
struct I;

trait Element {}
impl Element for I {}
impl Element for K {}
impl Element for S {}
impl Element for X {}
impl Element for Y {}
impl Element for Z {}

// * list

struct Seq<A: Element, B> {
    _phantom: PhantomData<(A, B)>,
}

struct Empty;

trait List {}
impl<A: Element, Rest: List> List for Seq<A, Rest> {}
impl List for Empty {}

// * groups

struct Group<Inner: List> {
    _phantom: PhantomData<Inner>,
}

impl<Inner: List> Element for Group<Inner> {}

// * SKI reduction

trait SKI<L: List> {
    type Result;

    fn reduce(PhantomData: PhantomData<L>) -> PhantomData<Self::Result> {
        PhantomData
    }
}

// * I reduction rules
impl SKI<Seq<I, Empty>> for () {
    type Result = Seq<I, Empty>;
}

impl<A, Rest> SKI<Seq<I, Seq<A, Rest>>> for ()
where
    A: Element,
    Rest: List,
    (): SKI<Seq<A, Rest>>,
    // Self::Result,
{
    type Result = <() as SKI<Seq<A, Rest>>>::Result;
}

// * K reduction rules
impl<A, B, Rest> SKI<Seq<K, Seq<A, Seq<B, Rest>>>> for ()
where
    A: Element,
    B: Element,
    Rest: List,
    (): SKI<Seq<A, Rest>>,
{
    type Result = <() as SKI<Seq<A, Rest>>>::Result;
}

impl<A> SKI<Seq<K, Seq<A, Empty>>> for ()
where
    A: Element,
{
    type Result = Seq<K, Seq<A, Empty>>;
}

impl SKI<Seq<K, Empty>> for () {
    type Result = Seq<K, Empty>;
}

// * S reduction rules
impl<A, B, C, Rest> SKI<Seq<S, Seq<A, Seq<B, Seq<C, Rest>>>>> for ()
where
    A: Element,
    B: Element,
    C: Element,
    Rest: List,
    (): SKI<Seq<A, Seq<C, Seq<Group<Seq<B, Seq<C, Empty>>>, Empty>>>>,
{
    type Result = <() as SKI<Seq<A, Seq<C, Seq<Group<Seq<B, Seq<C, Empty>>>, Empty>>>>>::Result;
}

// * Group reduction rule
impl<Expr, Rest> SKI<Seq<Group<Expr>, Rest>> for ()
where
    Expr: List,
    Rest: List,
    (): SKI<Expr>,
    // (): SKI<Seq<<() as SKI<Expr>>::Result, Rest>>,
{
    type Result = <() as SKI<Expr>>::Result;
    // type Result = <() as SKI<Seq<<() as SKI<Expr>>::Result, Rest>>>::Result;
}
