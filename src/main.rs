#![feature(type_ascription)]
use std::marker::PhantomData;

fn main() {
    // let () = <()>::reduce(PhantomData: PhantomData<I>);
    // let () = <()>::reduce(PhantomData: PhantomData<App<I, I>>);
    // let () = <()>::reduce(PhantomData: PhantomData<App<App<K, I>, I>>);
    // let () = <()>::reduce(PhantomData: PhantomData<App<K, I>>);
    // let () = <()>::reduce(PhantomData: PhantomData<App<App<App<S, X>, Y>, Z>>);
    // let PhantomData: PhantomData<()> = <()>::reduce(PhantomData: PhantomData<App<App<I, K>, I>>);
    let PhantomData: PhantomData<()> = <()>::reduce(PhantomData: PhantomData<App<App<I, I>, I>>);
    // let PhantomData: PhantomData<()> =
    //     <()>::reduce(PhantomData: PhantomData<App<App<App<I, K>, I>, I>>);
}

trait SKI<Input> {
    type Result;

    fn reduce(PhantomData: PhantomData<Input>) -> PhantomData<Self::Result> {
        PhantomData
    }
}

struct X;
struct Y;
struct Z;

struct S;
struct K;
struct I;

struct App<A, B> {
    _phantom: PhantomData<(A, B)>,
}

// * terminal rules

impl SKI<K> for () {
    type Result = K;
}

impl SKI<I> for () {
    type Result = I;
}

impl<A> SKI<App<K, A>> for () {
    type Result = App<K, A>;
}

// * non-terminal rules

impl<A> SKI<App<I, A>> for () {
    type Result = A;
}

impl<A, B> SKI<App<App<K, A>, B>> for () {
    type Result = A;
}

impl<A, B, C> SKI<App<App<App<S, A>, B>, C>> for () {
    type Result = App<App<A, C>, App<B, C>>;
}

impl<A, B> SKI<App<App<I, A>, B>> for ()
where
    (): SKI<App<A, B>>,
{
    type Result = <() as SKI<App<A, B>>>::Result;
}
