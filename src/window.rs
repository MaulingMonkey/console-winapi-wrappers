use crate::*;

use winapi::shared::windef::HWND;
use winapi::um::wincon::*;

use std::ffi::*;
use std::os::windows::prelude::*;
use std::io;



/// \[[GetConsoleOriginalTitleW]\] Retrieves the original title for the current console window.
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
/// [GetConsoleOriginalTitleW]: https://docs.microsoft.com/en-us/windows/console/getconsoletitle
pub fn get_console_original_title() -> io::Result<OsString> {
    let mut buffer = [0u16; 64 * 1024];
    match unsafe { GetConsoleOriginalTitleW(buffer.as_mut_ptr(), buffer.len() as _) } {
        0 => Err(io::Error::last_os_error()),
        n => Ok(OsString::from_wide(&buffer[.. n as _])),
    }
}

/// \[[GetConsoleTitleW]\] Retrieves the title for the current console window.
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
/// [GetConsoleTitleW]: https://docs.microsoft.com/en-us/windows/console/getconsoletitle
pub fn get_console_title() -> io::Result<OsString> {
    let mut buffer = [0u16; 64 * 1024];
    match unsafe { GetConsoleTitleW(buffer.as_mut_ptr(), buffer.len() as _) } {
        0 => Err(io::Error::last_os_error()),
        n => Ok(OsString::from_wide(&buffer[.. n as _])),
    }
}

/// \[[GetConsoleWindow]\] Retrieves the window handle used by the console associated with the calling process.
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
/// [GetConsoleWindow]: https://docs.microsoft.com/en-us/windows/console/getconsolewindow
pub fn get_console_window() -> io::Result<HWND> {
    match unsafe { GetConsoleWindow() } {
        hwnd if hwnd.is_null()  => Err(io::Error::last_os_error()),
        hwnd                    => Ok(hwnd),
    }
}

/// \[[GetLargestConsoleWindowSize]\] Retrieves the size of the largest possible console window, based on the current font and the size of the display.
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
/// [GetLargestConsoleWindowSize]:  https://docs.microsoft.com/en-us/windows/console/getlargestconsolewindowsize
pub fn get_largest_console_window_size(console_output: &impl AsConsoleOutputHandle) -> io::Result<COORD> {
    match unsafe { GetLargestConsoleWindowSize(console_output.as_raw_handle().cast()) } {
        COORD { X: 0, Y: 0 }    => Err(io::Error::last_os_error()),
        c                       => Ok(c),
    }
}

/// \[[SetConsoleTitleW]\] Sets the title for the current console window.
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
/// [SetConsoleTitleW]: https://docs.microsoft.com/en-us/windows/console/setconsoletitle
pub fn set_console_title(title: impl AsRef<OsStr>) -> io::Result<()> {
    let title = title.as_ref().encode_wide().chain(Some(0)).collect::<Vec<_>>();
    succeeded_to_result(unsafe { SetConsoleTitleW(title.as_ptr()) })
}

/// \[[SetConsoleWindowInfo]\] Sets the current size and position of a console screen buffer's window.
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
/// [SetConsoleWindowInfo]: https://docs.microsoft.com/en-us/windows/console/setconsolewindowinfo
pub fn set_console_window_info(console_output: &mut impl AsConsoleOutputHandle, absolute: bool, console_window: impl IntoSmallRect) -> io::Result<()> {
    succeeded_to_result(unsafe { SetConsoleWindowInfo(console_output.as_raw_handle().cast(), absolute.into(), &console_window.into_small_rect()) })
}
