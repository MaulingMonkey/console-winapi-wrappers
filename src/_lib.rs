#![doc = include_str!("../doc/console-function-mapping.md")]
#![cfg(windows)]
#![allow(unused_unsafe)]



mod _impl;      pub(crate) use _impl::*;
mod _traits;    pub use _traits::*;
mod alloc;      pub use alloc::*;
