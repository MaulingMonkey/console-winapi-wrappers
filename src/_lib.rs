#![doc = include_str!("../doc/console-function-mapping.md")]
#![cfg(windows)]
#![allow(unused_unsafe)]



mod _impl;      pub(crate) use _impl::*;
mod _traits;    pub use _traits::*;
mod alloc;      pub use alloc::*;
mod charattrib; pub use charattrib::*;
mod codepage;   pub use codepage::*;
mod font;       pub use font::*;
mod handles;    pub use handles::*;
mod io;         pub use io::*;
mod mode;       pub use mode::*;
