#![feature(type_ascription)]
use std::marker::PhantomData;

fn main() {
    let () = <()>::add(
        PhantomData: PhantomData<S<Z>>,
        PhantomData: PhantomData<S<Z>>,
    );
}

struct Z;
struct S<T: Nat>(PhantomData<T>);

trait Nat {}

impl<N: Nat> Nat for S<N> {}
impl Nat for Z {}

trait Add<A, B> {
    type R;

    fn add(_: PhantomData<A>, _: PhantomData<B>) -> PhantomData<Self::R> {
        PhantomData
    }
}

impl<B> Add<Z, B> for () {
    type R = B;
}

impl<A, B> Add<S<A>, B> for ()
where
    A: Nat,
    B: Nat,
    (): Add<A, S<B>>,
{
    type R = <() as Add<A, S<B>>>::R;
}
