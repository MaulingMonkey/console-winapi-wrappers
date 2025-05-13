#![doc = include_str!("../Readme.md")]
#![cfg(windows)]
#![allow(unused_unsafe)]



#[doc = include_str!("../doc/console-function-mapping.md")]
#[cfg(doc)] pub mod _console_function_mapping {}

#[path = r"values\_values.rs"]                      mod values;                 pub use values::*;

mod _impl;      pub(crate) use _impl::*;
mod _traits;    pub use _traits::*;
mod alias;      pub use alias::*;
mod alloc;      pub use alloc::*;
mod charattrib; pub use charattrib::*;
mod codepage;   pub use codepage::*;
mod font;       pub use font::*;
mod handles;    pub use handles::*;
mod io;         pub use io::*;
mod mode;       pub use mode::*;
mod text;       pub use text::*;
mod window;     pub use window::*;
