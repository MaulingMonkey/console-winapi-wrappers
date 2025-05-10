use crate::*;

use winapi::shared::windef::HWND;
use winapi::um::wincon::*;

use std::ffi::*;
use std::os::windows::prelude::*;
use std::io;



#[doc(alias = "GetConsoleOriginalTitle")]
#[doc(alias = "GetConsoleOriginalTitleW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/getconsoleoriginaltitle)\]
/// Retrieves the original title for the current console window.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::ffi::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// let title : OsString = get_console_original_title()?;
/// # Ok(())
/// # })();
/// ```
///
pub fn get_console_original_title() -> io::Result<OsString> {
    let mut buffer = [0u16; 64 * 1024];
    match unsafe { GetConsoleOriginalTitleW(buffer.as_mut_ptr(), buffer.len() as _) } {
        0 => Err(io::Error::last_os_error()),
        n => Ok(OsString::from_wide(&buffer[.. n as _])),
    }
}

#[doc(alias = "GetConsoleTitle")]
#[doc(alias = "GetConsoleTitleW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/getconsoletitle)\]
/// Retrieves the title for the current console window.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::ffi::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// let title : OsString = get_console_title()?;
/// # Ok(())
/// # })();
/// ```
///
pub fn get_console_title() -> io::Result<OsString> {
    let mut buffer = [0u16; 64 * 1024];
    match unsafe { GetConsoleTitleW(buffer.as_mut_ptr(), buffer.len() as _) } {
        0 => Err(io::Error::last_os_error()),
        n => Ok(OsString::from_wide(&buffer[.. n as _])),
    }
}

#[doc(alias = "GetConsoleWindow")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/getconsolewindow)\]
/// Retrieves the current console's window handle.
///
/// Note that this almost certainly belongs to another process (`cmd.exe`, which may be a parent, child, or unrelated process.)
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use winapi::shared::windef::HWND;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// let hwnd : HWND = get_console_window()?;
/// # Ok(())
/// # })();
/// ```
///
pub fn get_console_window() -> io::Result<HWND> {
    match unsafe { GetConsoleWindow() } {
        hwnd if hwnd.is_null()  => Err(io::Error::last_os_error()),
        hwnd                    => Ok(hwnd),
    }
}

#[doc(alias = "GetLargestConsoleWindowSize")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/getlargestconsolewindowsize)\]
/// Calculates the largest possible window, given the current font &amp; display.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use winapi::um::wincontypes::COORD;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// let s : COORD = get_largest_console_window_size(&stdout())?;
/// # Ok(())
/// # })();
/// ```
///
pub fn get_largest_console_window_size(console_output: &impl AsConsoleOutputHandle) -> io::Result<COORD> {
    match unsafe { GetLargestConsoleWindowSize(console_output.as_raw_handle().cast()) } {
        COORD { X: 0, Y: 0 }    => Err(io::Error::last_os_error()),
        c                       => Ok(c),
    }
}

#[doc(alias = "SetConsoleTitle")]
#[doc(alias = "SetConsoleTitleW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/setconsoletitle)\]
/// Sets the title for the current console window.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// set_console_title("new title")?;
/// # Ok(())
/// # })();
/// ```
///
pub fn set_console_title(title: impl AsRef<OsStr>) -> io::Result<()> {
    let title = title.as_ref().encode_wide().chain(Some(0)).collect::<Vec<_>>();
    succeeded_to_result(unsafe { SetConsoleTitleW(title.as_ptr()) })
}

#[doc(alias = "SetConsoleWindowInfo")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/setconsolewindowinfo)\]
/// Sets the current size and position of a console screen buffer's window.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// set_console_window_info(&mut stdout(), false, (0,0) .. (80,25))?;
/// # Ok(())
/// # })();
/// ```
///
pub fn set_console_window_info(console_output: &mut impl AsConsoleOutputHandle, absolute: bool, console_window: impl Into<SmallRect>) -> io::Result<()> {
    succeeded_to_result(unsafe { SetConsoleWindowInfo(console_output.as_raw_handle().cast(), absolute.into(), &console_window.into().into()) })
}
