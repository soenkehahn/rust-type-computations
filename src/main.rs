use std::marker::PhantomData;

fn main() {
    let phantom: PhantomData<()> = PhantomData;
    let () = <()>::reduce(phantom);
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

impl<A, Rest> SKI<Seq<I, Seq<A, Rest>>> for ()
where
    A: Element,
    Rest: List,
    (): SKI<Seq<A, Rest>>,
{
    type Result = <() as SKI<Seq<A, Rest>>>::Result;
}

impl SKI<Seq<I, Empty>> for () {
    type Result = Seq<I, Empty>;
}
