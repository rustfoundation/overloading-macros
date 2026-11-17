//! Test C++ [std::stack][stack] overloads that can't be represented in Rust, and record the macro diagnostics.
//! Working overloads are in [stack.rs].
//!
//! [stack]:  https://en.cppreference.com/cpp/container/stack
//! [stack.rs]: https://github.com/rustfoundation/overloading-macros/blob/main/cpp-overload-test/src/stdcpp/stack.rs
//!
//! The `cpp!` macro doesn't work here.

#![feature(splat, tuple_trait, c_size_t)]
#![allow(incomplete_features)]
#![allow(unused_braces)]

use splat_overload::overload;
use std::ffi::c_void;
use std::marker::PhantomData;

#[expect(dead_code, reason = "Incompatible overloads can't be used")]
struct StdStack(*mut c_void, PhantomData<T>);

type T = std::ffi::c_int;

overload! {
    impl StdStack {
        /// ### Incompatibilities
        ///
        /// cpp has two `two` top functions with the same name, `T& top()` return an item you can change,
        /// and `const T& top() const` return a read only one. they only differ by const,
        /// which in Rust is `&mut self` reciever vs a `&self ` receiver.
        ///
        /// the overload! cant express this. it turns all overloads with the same name into one
        /// real function, that sends each call to the right version. and one function can
        /// only have one kind of receiver , never `&self` and `&mut self` at the same time.
        ///
        /// ### Workarounds
        ///
        /// The simplest fix would be to not overload at all and use the two names
        /// `top()` for read only version and `top_mut()` for the changeable one.
        /// Different names means theres no overloading is needed , so macro isn't
        /// involved. It will not be same as C++, where both functions are
        /// called top. But its how Rust normally does it, Vec has first() and first_mut()
        /// ANd Rust users would find it natural.
        ///
        /// The other option is like in the `forward_list::front` , take the stack as a normal
        /// argument and not as self, like `top(this: &'static StdStack)` and
        /// `top(this: &'static mut StdStack)` . but the thing is the macro, doesnt support
        /// lifetimes , so the reference has to be static, and the stack has to be leaked ,
        /// which is very awkward to use and the callers write `StdStack::top(&stack)`
        /// and not `stack.top()`
        fn top(&self) where T:Sized {
            todo!()
        }

        fn top(&mut self) where T: Sized {
            todo!()
        }
    }
}

pub fn main() {}
