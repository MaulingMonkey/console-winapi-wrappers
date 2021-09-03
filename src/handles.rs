use crate::*;

use std::fmt::Debug;
use std::io::*;
use std::os::windows::io::*;



/// [AsConsoleInputHandle] | [AsConsoleOutputHandle]
pub unsafe trait AsConsoleHandle : AsRawHandle {
    /// [InputMode] | [OutputMode] DWORD wrapper
    type Mode : Copy + Debug + From<u32> + Into<u32>;
}

unsafe impl AsConsoleHandle for Stdin               { type Mode = InputMode; }
unsafe impl AsConsoleHandle for StdinLock<'_>       { type Mode = InputMode; }
unsafe impl AsConsoleHandle for Stderr              { type Mode = OutputMode; }
unsafe impl AsConsoleHandle for Stdout              { type Mode = OutputMode; }
unsafe impl AsConsoleHandle for StderrLock<'_>      { type Mode = OutputMode; }
unsafe impl AsConsoleHandle for StdoutLock<'_>      { type Mode = OutputMode; }
unsafe impl AsConsoleHandle for ConsoleScreenBuffer { type Mode = OutputMode; }



/// std::io::{[Stdin] | [StdinLock]}
pub trait AsConsoleInputHandle  : AsConsoleHandle<Mode = InputMode > {
}

/// std::io::{[Stderr] | [Stdout] | [StderrLock] | [StdoutLock]} | [ConsoleScreenBuffer]
pub trait AsConsoleOutputHandle : AsConsoleHandle<Mode = OutputMode> {
}

impl<T: AsConsoleHandle<Mode = InputMode >> AsConsoleInputHandle  for T {}
impl<T: AsConsoleHandle<Mode = OutputMode>> AsConsoleOutputHandle for T {}
