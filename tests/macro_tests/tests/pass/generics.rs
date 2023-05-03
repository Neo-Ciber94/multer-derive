use multer_derive::FromMultipart;
use std::marker::PhantomData;

#[derive(FromMultipart)]
struct MyStruct<'lifetime, A, B, C, D, E> {
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    _marker: &'lifetime PhantomData<()>,
}

fn main() {}
