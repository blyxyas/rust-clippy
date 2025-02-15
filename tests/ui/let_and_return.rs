//@revisions: edition2021 edition2024
//@[edition2021] edition:2021
//@[edition2024] edition:2024

#![allow(unused)]
#![warn(clippy::let_and_return)]

use std::cell::RefCell;

fn test() -> i32 {
    let _y = 0; // no warning
    let x = 5;
    x
    //~^ let_and_return
}

fn test_inner() -> i32 {
    if true {
        let x = 5;
        x
        //~^ let_and_return
    } else {
        0
    }
}

fn test_nowarn_1() -> i32 {
    let mut x = 5;
    x += 1;
    x
}

fn test_nowarn_2() -> i32 {
    let x = 5;
    x + 1
}

fn test_nowarn_3() -> (i32, i32) {
    // this should technically warn, but we do not compare complex patterns
    let (x, y) = (5, 9);
    (x, y)
}

fn test_nowarn_4() -> i32 {
    // this should technically warn, but not b/c of clippy::let_and_return, but b/c of useless type
    let x: i32 = 5;
    x
}

fn test_nowarn_5(x: i16) -> u16 {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let x = x as u16;
    x
}

// False positive example
trait Decode {
    fn decode<D: std::io::Read>(d: D) -> Result<Self, ()>
    where
        Self: Sized;
}

macro_rules! tuple_encode {
    ($($x:ident),*) => (
        impl<$($x: Decode),*> Decode for ($($x),*) {
            #[inline]
            #[allow(non_snake_case)]
            fn decode<D: std::io::Read>(mut d: D) -> Result<Self, ()> {
                // Shouldn't trigger lint
                Ok(($({let $x = Decode::decode(&mut d)?; $x }),*))
            }
        }
    );
}

fn issue_3792() -> String {
    use std::io::{self, BufRead, Stdin};

    let stdin = io::stdin();
    // `Stdin::lock` returns `StdinLock<'static>` so `line` doesn't borrow from `stdin`
    // https://github.com/rust-lang/rust/pull/93965
    let line = stdin.lock().lines().next().unwrap().unwrap();
    line
    //~^ let_and_return
}

tuple_encode!(T0, T1, T2, T3, T4, T5, T6, T7);

mod no_lint_if_stmt_borrows {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};
    struct Bar;

    impl Bar {
        fn new() -> Self {
            Bar {}
        }
        fn baz(&self) -> u32 {
            0
        }
    }

    fn issue_3324(value: Weak<RefCell<Bar>>) -> u32 {
        let value = value.upgrade().unwrap();
        let ret = value.borrow().baz();
        ret
        //~[edition2024]^ let_and_return
    }

    fn borrows_in_closure(value: Weak<RefCell<Bar>>) -> u32 {
        fn f(mut x: impl FnMut() -> u32) -> impl FnMut() -> u32 {
            x
        }

        let value = value.upgrade().unwrap();
        let ret = f(|| value.borrow().baz())();
        ret
        //~[edition2024]^ let_and_return
    }

    mod free_function {
        struct Inner;

        struct Foo<'a> {
            inner: &'a Inner,
        }

        impl Drop for Foo<'_> {
            fn drop(&mut self) {}
        }

        impl<'a> Foo<'a> {
            fn new(inner: &'a Inner) -> Self {
                Self { inner }
            }

            fn value(&self) -> i32 {
                42
            }
        }

        fn some_foo(inner: &Inner) -> Foo<'_> {
            Foo { inner }
        }

        fn test() -> i32 {
            let x = Inner {};
            let value = some_foo(&x).value();
            value
            //~[edition2024]^ let_and_return
        }

        fn test2() -> i32 {
            let x = Inner {};
            let value = Foo::new(&x).value();
            value
            //~[edition2024]^ let_and_return
        }
    }
}

mod issue_5729 {
    use std::sync::Arc;

    trait Foo {}

    trait FooStorage {
        fn foo_cloned(&self) -> Arc<dyn Foo>;
    }

    struct FooStorageImpl<T: Foo> {
        foo: Arc<T>,
    }

    impl<T: Foo + 'static> FooStorage for FooStorageImpl<T> {
        fn foo_cloned(&self) -> Arc<dyn Foo> {
            let clone = Arc::clone(&self.foo);
            clone
            //~^ let_and_return
        }
    }
}

mod issue_11335 {
    pub enum E<T> {
        A(T),
        B(T),
    }

    impl<T> E<T> {
        pub fn inner(&self) -> &T {
            let result = match self {
                E::A(x) => x,
                E::B(x) => x,
            };

            result
            //~^ let_and_return
        }
    }
}

// https://github.com/rust-lang/rust-clippy/issues/11167
macro_rules! fn_in_macro {
    ($b:block) => {
        fn f() -> usize $b
    }
}
fn_in_macro!({
    return 1;
});

fn issue9150() -> usize {
    let x = 1;
    #[cfg(any())]
    panic!("can't see me");
    x
}

fn issue12801() {
    fn left_is_if() -> String {
        let s = if true { "a".to_string() } else { "b".to_string() } + "c";
        s
        //~^ let_and_return
    }

    fn no_par_needed() -> String {
        let s = "c".to_string() + if true { "a" } else { "b" };
        s
        //~^ let_and_return
    }

    fn conjunctive_blocks() -> String {
        let s = { "a".to_string() } + "b" + { "c" } + "d";
        s
        //~^ let_and_return
    }

    #[allow(clippy::overly_complex_bool_expr)]
    fn other_ops() {
        let _ = || {
            let s = if true { 2 } else { 3 } << 4;
            s
            //~^ let_and_return
        };
        let _ = || {
            let s = { true } || { false } && { 2 <= 3 };
            s
            //~^ let_and_return
        };
    }
}

fn issue14164() -> Result<u32, ()> {
    let v = std::cell::RefCell::new(Some(vec![1]));
    let r = match &*v.borrow() {
        Some(v) => Ok(Ok(v[0])),
        None => Ok(Ok(0)),
    }?;
    r
    //~[edition2024]^ let_and_return
}

fn main() {}
