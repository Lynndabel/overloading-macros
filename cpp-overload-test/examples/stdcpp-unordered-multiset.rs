//! Test C++ [std::unordered_multiset][unordered_multiset] overloads that can't be represented in Rust.
//! Working overloads are in [unordered_multiset.rs].
//!
//! [unordered_multiset]: https://en.cppreference.com/cpp/container/unordered_multiset
//! [unordered_multiset.rs]: https://github.com/rustfoundation/overloading-macros/blob/main/cpp-overload-test/src/stdcpp/unordered_multiset.rs
//!
//! The `cpp!` macro doesn't work here because examples are built separately from `lib.rs`.
//!
//! ### Incompatibilities
//!
//! These overloads require C++ template types like `std::hash<int>` which don't have Rust equivalents.
//! Rust function pointers can't be converted to these C++ functor types.
//!
//! ### Workarounds
//!
//! Would need template specialization or C++ wrapper classes to accept function pointers.
//! This is complex and beyond the current scope.

#![feature(splat, tuple_trait, c_size_t)]
#![allow(incomplete_features)]
#![allow(unused_braces)]

use splat_overload::overload;
use std::ffi::{c_size_t, c_void};
use std::marker::PhantomData;

#[expect(dead_code, reason = "Incompatible overloads can't be used")]
struct StdUnorderedMultiset(*mut c_void, PhantomData<T>);

type T = std::ffi::c_int;

overload! {
    impl StdUnorderedMultiset {
        /// Custom hash function - can't implement because C++ expects `std::hash<int>`, not function pointers.
        fn new(_bucket_count: c_size_t, _hash_fn: extern "C" fn(&T) -> c_size_t) -> StdUnorderedMultiset {
            todo!()
        }

        /// Custom key equality function - same limitation as custom hash.
        fn new(_bucket_count: c_size_t, _key_equal: extern "C" fn(&T, &T) -> bool) -> StdUnorderedMultiset {
            todo!()
        }
    }
}

pub fn main() {}
