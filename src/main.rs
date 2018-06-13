#![recursion_limit = "32"]

use std::marker::PhantomData;

fn main() {
    // * concatenation
    // let () = <() as Concat<Empty, Seq<I, Empty>>>::concat(PhantomData);
    // let () = <() as Concat<Seq<I, Empty>, Empty>>::concat(PhantomData);
    // let () = <() as Concat<Seq<I, Empty>, Seq<K, Empty>>>::concat(PhantomData);
    // let () = <() as SKI<Seq<I, Seq<I, Empty>>>>::reduce(PhantomData);
    // let () = <() as SKI<Seq<K, Seq<I, Seq<I, Empty>>>>>::reduce(PhantomData);
    // let () = <() as SKI<Seq<K, Seq<I, Empty>>>>::reduce(PhantomData);
    // let () = <() as SKI<Seq<K, Empty>>>::reduce(PhantomData);
    // let () = <() as SKI<Seq<S, Seq<I, Seq<I, Seq<I, Empty>>>>>>::reduce(PhantomData);
    // let () = <() as SKI<Seq<S, Seq<I, Seq<I, Seq<I, Seq<I, Empty>>>>>>>::reduce(PhantomData);
    // let () = <() as SKI<Seq<Group<Seq<I, Empty>>, Seq<K, Empty>>>>::reduce(PhantomData);
    // let () = <() as SKI<Seq<S, Seq<X, Seq<Y, Seq<Z, Seq<K, Empty>>>>>>>::reduce(PhantomData);

    // * flip
    // let () = <() as SKI<
    //     Seq<
    //         S,
    //         Seq<
    //             Group<Seq<K, Seq<Group<Seq<S, Seq<I, Empty>>>, Empty>>>,
    //             Seq<K, Seq<S, Seq<K, Empty>>>,
    //         >,
    //     >,
    // >>::reduce(PhantomData);

    // * non-terminating:
    let () = <() as SKI<
        Seq<S, Seq<I, Seq<I, Seq<Group<Seq<S, Seq<I, Seq<I, Empty>>>>, Empty>>>>,
    >>::reduce(PhantomData);
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

struct Seq<A: Element, B: List> {
    _phantom: PhantomData<(A, B)>,
}

struct Empty;

trait List {}
impl<A: Element, Rest: List> List for Seq<A, Rest> {}
impl List for Empty {}

// * concatenation

trait Concat<A: List, B: List> {
    type Result: List;

    fn concat(PhantomData: PhantomData<(A, B)>) -> PhantomData<Self::Result> {
        PhantomData
    }
}

impl<B: List> Concat<Empty, B> for () {
    type Result = B;
}

impl<A: Element, Rest: List, L: List> Concat<Seq<A, Rest>, L> for ()
where
    (): Concat<Rest, L>,
{
    type Result = Seq<A, <() as Concat<Rest, L>>::Result>;
}

// * groups

struct Group<Inner: List> {
    _phantom: PhantomData<Inner>,
}

impl<Inner: List> Element for Group<Inner> {}

// * SKI reduction

trait SKI<L: List> {
    type Result: List;

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
    (): SKI<Seq<A, Empty>>,
{
    type Result = Seq<K, Seq<Group<<() as SKI<Seq<A, Empty>>>::Result>, Empty>>;
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
    (): SKI<Seq<A, Seq<C, Seq<Group<Seq<B, Seq<C, Empty>>>, Rest>>>>,
{
    type Result = <() as SKI<Seq<A, Seq<C, Seq<Group<Seq<B, Seq<C, Empty>>>, Rest>>>>>::Result;
}

impl SKI<Seq<S, Empty>> for () {
    type Result = Seq<S, Empty>;
}

impl<A> SKI<Seq<S, Seq<A, Empty>>> for ()
where
    A: Element,
{
    type Result = Seq<S, Seq<A, Empty>>;
}

impl<A, B> SKI<Seq<S, Seq<A, Seq<B, Empty>>>> for ()
where
    A: Element,
    B: Element,
{
    type Result = Seq<S, Seq<A, Seq<B, Empty>>>;
}

// * Group reduction rule

impl<Expr, Rest> SKI<Seq<Group<Expr>, Rest>> for ()
where
    Expr: List,
    Rest: List,
    (): SKI<Expr>,
    (): Concat<<() as SKI<Expr>>::Result, Rest>,
    (): SKI<<() as Concat<<() as SKI<Expr>>::Result, Rest>>::Result>,
{
    type Result = <() as SKI<<() as Concat<<() as SKI<Expr>>::Result, Rest>>::Result>>::Result;
}
