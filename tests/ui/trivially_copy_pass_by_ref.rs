//@normalize-stderr-test: "\(\d+ byte\)" -> "(N byte)"
//@normalize-stderr-test: "\(limit: \d+ byte\)" -> "(limit: N byte)"
#![deny(clippy::trivially_copy_pass_by_ref)]
#![allow(
    clippy::disallowed_names,
    clippy::extra_unused_lifetimes,
    clippy::needless_lifetimes,
    clippy::needless_pass_by_ref_mut,
    clippy::redundant_field_names,
    clippy::uninlined_format_args
)]

#[derive(Copy, Clone)]
struct Foo(u32);

#[derive(Copy, Clone)]
struct Bar([u8; 24]);

#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

struct FooRef<'a> {
    foo: &'a Foo,
}

type Baz = u32;

fn good(a: &mut u32, b: u32, c: &Bar) {}

fn good_return_implicit_lt_ref(foo: &Foo) -> &u32 {
    &foo.0
}

#[allow(clippy::needless_lifetimes)]
fn good_return_explicit_lt_ref<'a>(foo: &'a Foo) -> &'a u32 {
    &foo.0
}

fn good_return_implicit_lt_struct(foo: &Foo) -> FooRef {
    FooRef { foo }
}

#[allow(clippy::needless_lifetimes)]
fn good_return_explicit_lt_struct<'a>(foo: &'a Foo) -> FooRef<'a> {
    FooRef { foo }
}

fn bad(x: &u32, y: &Foo, z: &Baz) {}
//~^ ERROR: this argument (4 byte) is passed by reference, but would be more efficient if passed by
//~| ERROR: this argument (4 byte) is passed by reference, but would be more efficient if passed by
//~| ERROR: this argument (4 byte) is passed by reference, but would be more efficient if passed by

impl Foo {
    fn good(self, a: &mut u32, b: u32, c: &Bar) {}

    fn good2(&mut self) {}

    fn bad(&self, x: &u32, y: &Foo, z: &Baz) {}
    //~^ ERROR: this argument (4 byte) is passed by reference, but would be more efficient if passed by
    //~| ERROR: this argument (4 byte) is passed by reference, but would be more efficient if passed by
    //~| ERROR: this argument (4 byte) is passed by reference, but would be more efficient if passed by
    //~| ERROR: this argument (4 byte) is passed by reference, but would be more efficient if passed by

    fn bad2(x: &u32, y: &Foo, z: &Baz) {}
    //~^ ERROR: this argument (4 byte) is passed by reference, but would be more efficient if passed by
    //~| ERROR: this argument (4 byte) is passed by reference, but would be more efficient if passed by
    //~| ERROR: this argument (4 byte) is passed by reference, but would be more efficient if passed by

    fn bad_issue7518(self, other: &Self) {}
    //~^ ERROR: this argument (4 byte) is passed by reference, but would be more efficient if
}

impl AsRef<u32> for Foo {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}

impl Bar {
    fn good(&self, a: &mut u32, b: u32, c: &Bar) {}

    fn bad2(x: &u32, y: &Foo, z: &Baz) {}
    //~^ ERROR: this argument (4 byte) is passed by reference, but would be more efficient if
    //~| ERROR: this argument (4 byte) is passed by reference, but would be more efficient if
    //~| ERROR: this argument (4 byte) is passed by reference, but would be more efficient if
}

trait MyTrait {
    fn trait_method(&self, foo: &Foo);
    //~^ ERROR: this argument (4 byte) is passed by reference, but would be more efficient if
}

pub trait MyTrait2 {
    fn trait_method2(&self, color: &Color);
}

trait MyTrait3 {
    #[expect(clippy::trivially_copy_pass_by_ref)]
    fn trait_method(&self, foo: &Foo);
}

// Trait impls should not warn
impl MyTrait3 for Foo {
    fn trait_method(&self, foo: &Foo) {
        unimplemented!()
    }
}

mod issue3992 {
    pub trait A {
        #[allow(clippy::trivially_copy_pass_by_ref)]
        fn a(b: &u16) {}
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn c(d: &u16) {}
}

mod issue5876 {
    // Don't lint here as it is always inlined
    #[inline(always)]
    fn foo_always(x: &i32) {
        println!("{}", x);
    }

    #[inline(never)]
    fn foo_never(x: &i32) {
        //~^ ERROR: this argument (4 byte) is passed by reference, but would be more efficient if passed by
        println!("{}", x);
    }

    #[inline]
    fn foo(x: &i32) {
        //~^ ERROR: this argument (4 byte) is passed by reference, but would be more efficient if passed by
        println!("{}", x);
    }
}

fn ref_to_opt_ref_implicit(x: &u32) -> Option<&u32> {
    Some(x)
}

fn ref_to_opt_ref_explicit<'a>(x: &'a u32) -> Option<&'a u32> {
    Some(x)
}

fn with_constraint<'a, 'b: 'a>(x: &'b u32, y: &'a u32) -> &'a u32 {
    if true { x } else { y }
}

async fn async_implicit(x: &u32) -> &u32 {
    x
}

async fn async_explicit<'a>(x: &'a u32) -> &'a u32 {
    x
}

fn unrelated_lifetimes<'a, 'b>(_x: &'a u32, y: &'b u32) -> &'b u32 {
    //~^ ERROR: this argument (4 byte) is passed by reference, but would be more efficient if passed by
    y
}

fn return_ptr(x: &u32) -> *const u32 {
    x
}

fn return_field_ptr(x: &(u32, u32)) -> *const u32 {
    &x.0
}

fn return_field_ptr_addr_of(x: &(u32, u32)) -> *const u32 {
    core::ptr::addr_of!(x.0)
}
