use std::marker::PhantomData;

fn main() {
    let phantom: PhantomData<()> = PhantomData;
    let () = <()>::reduce(phantom);
    // let () = <()>::reduce(PhantomData: PhantomData<Seq<I, Empty>>);
    // let () = <()>::reduce(PhantomData: PhantomData<Seq<I, Seq<I, Empty>>>);
    // let () = <()>::reduce(PhantomData: PhantomData<Seq<K, Seq<I, Seq<I, Empty>>>>);
    // let () = <()>::reduce(PhantomData: PhantomData<Seq<K, Seq<I, Empty>>>);
    // let () = <()>::reduce(PhantomData: PhantomData<Seq<S, Seq<X, Seq<Y, Seq<Z, Empty>>>>>);
    // let PhantomData: PhantomData<()> =
    //     <()>::reduce(PhantomData: PhantomData<Seq<I, Seq<K, Seq<I, Empty>>>>);
    // let PhantomData: PhantomData<()> =
    //     <()>::reduce(PhantomData: PhantomData<Seq<I, Seq<I, Seq<I, Empty>>>>);
    // let PhantomData: PhantomData<()> =
    //     <()>::reduce(PhantomData: PhantomData<App<App<App<I, K>, I>, I>>);
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

impl<A, B, Rest> SKI<Seq<K, Seq<A, Seq<B, Rest>>>> for ()
where
    A: Element,
    B: Element,
    Rest: List,
{
    type Result = Seq<A, Rest>;
}

impl<A, B, C, Rest> SKI<Seq<S, Seq<A, Seq<B, Seq<C, Rest>>>>> for ()
where
    A: Element,
    B: Element,
    C: Element,
    Rest: List,
{
    type Result = Seq<A, Seq<C, Seq<Group<Seq<B, Seq<C, Empty>>>, Rest>>>;
}

// * invariant rules

impl SKI<Seq<I, Empty>> for () {
    type Result = Seq<I, Empty>;
}

impl SKI<Seq<K, Seq<I, Empty>>> for () {
    type Result = Seq<K, Seq<I, Empty>>;
}
