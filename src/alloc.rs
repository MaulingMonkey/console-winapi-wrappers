use crate::*;

use winapi::shared::ntdef::HANDLE;
use winapi::um::consoleapi::*;
use winapi::um::handleapi::{self, *};
use winapi::um::processthreadsapi::*;
use winapi::um::wincon::*;
use winapi::um::winnt::*;

use std::io;
use std::ptr::*;
use std::os::windows::io::{AsRawHandle, RawHandle};




/// \[[AllocConsole]\] Allocates a new console for the calling process.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use winapi::shared::winerror::*;
/// let _ = alloc_console();
///
/// // Will fail if already attached
/// let err = alloc_console().unwrap_err();
/// assert_eq!(err.raw_os_error(), Some(ERROR_ACCESS_DENIED as _));
/// assert_eq!(err.kind(), std::io::ErrorKind::PermissionDenied);
/// ```
///
/// [AllocConsole]:                     https://docs.microsoft.com/en-us/windows/console/allocconsole
pub fn alloc_console() -> io::Result<()> {
    succeeded_to_result(unsafe{AllocConsole()})
}



/// \[[AttachConsole]\] Attaches the calling process to the console of the specified process.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use winapi::shared::winerror::*;
/// # use winapi::um::wincon::*;
/// let _ = attach_console(ATTACH_PARENT_PROCESS);
///
/// // Will fail if already attached
/// let err = attach_console(ATTACH_PARENT_PROCESS).unwrap_err();
/// assert_eq!(err.raw_os_error(), Some(ERROR_ACCESS_DENIED as _));
/// assert_eq!(err.kind(), std::io::ErrorKind::PermissionDenied);
/// ```
///
/// [AttachConsole]:                    https://docs.microsoft.com/en-us/windows/console/attachconsole
pub fn attach_console(process: impl IntoProcessId) -> io::Result<()> {
    let process = process.into_process_id();
    succeeded_to_result(unsafe{AttachConsole(process)})
}

/// \[[AttachConsole]\] Attaches the calling process to the console of the parent process.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use winapi::shared::winerror::*;
/// # use winapi::um::wincon::*;
/// let _ = attach_console_parent_process();
///
/// // Will fail if already attached
/// let err = attach_console_parent_process().unwrap_err();
/// assert_eq!(err.raw_os_error(), Some(ERROR_ACCESS_DENIED as _));
/// assert_eq!(err.kind(), std::io::ErrorKind::PermissionDenied);
/// ```
///
/// [AttachConsole]:                    https://docs.microsoft.com/en-us/windows/console/attachconsole
pub fn attach_console_parent_process() -> io::Result<()> {
    succeeded_to_result(unsafe{AttachConsole(ATTACH_PARENT_PROCESS)})
}



// TODO: psuedoconsoles?
// https://docs.microsoft.com/en-us/windows/console/creating-a-pseudoconsole-session
//
// | [CreatePseudoConsole]             | ![x] Allocates a new pseudoconsole for the calling process.
// | [ClosePseudoConsole]              | ![x] Closes a pseudoconsole from the given handle.
// | [ResizePseudoConsole]             | ![x]
//
// [CreatePseudoConsole]:              https://docs.microsoft.com/en-us/windows/console/createpseudoconsole
// [ClosePseudoConsole]:               https://docs.microsoft.com/en-us/windows/console/closepseudoconsole
// [ResizePseudoConsole]:              https://docs.microsoft.com/en-us/windows/console/resizepseudoconsole



/// \[[docs.microsoft.com]\] An owned win32 console screen buffer (2D grid of characters/glyphs)
///
/// [docs.microsoft.com]:        https://docs.microsoft.com/en-us/windows/console/createconsolescreenbuffer
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConsoleScreenBuffer {
    handle: HANDLE,
}

impl ConsoleScreenBuffer {
    /// <code>[CreateConsoleScreenBuffer]\(0, 0, NULL, CONSOLE_TEXTMODE_BUFFER, NULL\)</code> Creates a console screen buffer.
    ///
    /// ### Example
    /// ```
    /// # use maulingmonkey_console_winapi_wrappers::*;
    /// let csb = ConsoleScreenBuffer::new().unwrap();
    /// ```
    ///
    /// [CreateConsoleScreenBuffer]:        https://docs.microsoft.com/en-us/windows/console/createconsolescreenbuffer
    pub fn new() -> io::Result<Self> {
        match unsafe { CreateConsoleScreenBuffer(0, 0, null(), CONSOLE_TEXTMODE_BUFFER, null_mut()) } {
            handleapi::INVALID_HANDLE_VALUE => Err(io::Error::last_os_error()),
            handle if handle.is_null() => Err(io::Error::last_os_error()),
            handle => Ok(Self { handle })
        }
    }
}

impl AsRawHandle for ConsoleScreenBuffer {
    fn as_raw_handle(&self) -> RawHandle { self.handle.cast() }
}

/// \[[DuplicateHandle]\] Duplicates an object handle.
///
/// [DuplicateHandle]:  https://docs.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle
impl Clone for ConsoleScreenBuffer {
    fn clone(&self) -> Self {
        let current_process = unsafe { GetCurrentProcess() };
        let mut handle = null_mut();
        let succeeded = unsafe { DuplicateHandle(current_process, self.handle, current_process, &mut handle, 0, 0, DUPLICATE_SAME_ACCESS) };
        assert!(succeeded != 0, "ConsoleScreenBuffer::clone(): DuplicateHandle(...) failed: {:?}", io::Error::last_os_error());
        Self { handle }
    }
}

/// \[[CloseHandle]\] Closes an open object handle.
///
/// [CloseHandle]:  https://docs.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle
impl Drop for ConsoleScreenBuffer {
    fn drop(&mut self) {
        let _succeeded = unsafe { CloseHandle(self.handle) };
        debug_assert!(_succeeded != 0, "ConsoleScreenBuffer::drop(): CloseHandle(self.handle) failed: {:?}", io::Error::last_os_error());
    }
}

#[test] fn csb_traits() {
    let csb = ConsoleScreenBuffer::new().unwrap();
    let _h = csb.as_raw_handle(); // AsRawHandle
    let _csb2 = csb.clone(); // Clone
    std::mem::drop(csb); // Drop
}



/// \[[FreeConsole]\] Detaches the calling process from its console.
///
/// This function may return `Ok(())`, even if the calling process wasn't attached to a console, despite
/// [docs.microsoft.com]'s remarks suggesting that `ERROR_INVALID_PARAMETER` should occur.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// let _ = free_console();
/// ```
///
/// [docs.microsoft.com]:               https://docs.microsoft.com/en-us/windows/console/freeconsole
/// [FreeConsole]:                      https://docs.microsoft.com/en-us/windows/console/freeconsole
pub fn free_console() -> io::Result<()> {
    succeeded_to_result(unsafe{FreeConsole()})
}
