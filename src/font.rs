use crate::*;

use winapi::shared::minwindef::DWORD;
use winapi::um::wincon::*;

use std::convert::*;
use std::io;
use std::mem::{size_of_val, zeroed};



/// \[[GetConsoleFontSize]\] Retrieves the size of the font used by the specified console screen buffer.
///
/// ### Safety
///
/// * $5 says `nfont` is missing bounds checks or overflows on some version of Windows, Wine, or ReactOS.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// let font = get_current_console_font(&stdout(), false)?;
/// let font_size = unsafe { get_console_font_size(&stdout(), font.nFont)? };
/// assert_eq!(font_size.X, font.dwFontSize.X);
/// assert_eq!(font_size.Y, font.dwFontSize.Y);
/// # Ok(())
/// # })();
/// ```
///
/// [GetConsoleFontSize]: https://docs.microsoft.com/en-us/windows/console/getconsolefontsize
pub unsafe fn get_console_font_size(console_output: &impl AsConsoleOutputHandle, nfont: DWORD) -> io::Result<COORD> {
    match unsafe { GetConsoleFontSize(console_output.as_raw_handle().cast(), nfont) } {
        COORD { X: 0, Y: 0 }    => Err(io::Error::last_os_error()),
        coord                   => Ok(coord),
    }
}

/// \[[GetCurrentConsoleFont]\] Retrieves information about the current console font.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// let info = get_current_console_font(&stdout(), false)?;
/// # Ok(())
/// # })();
/// ```
///
/// [GetCurrentConsoleFont]:    https://docs.microsoft.com/en-us/windows/console/getcurrentconsolefont
pub fn get_current_console_font(console_output: &impl AsConsoleOutputHandle, maximum_window: bool) -> io::Result<CONSOLE_FONT_INFO> {
    let mut info : CONSOLE_FONT_INFO = unsafe { zeroed() };
    succeeded_to_result(unsafe { GetCurrentConsoleFont(console_output.as_raw_handle().cast(), maximum_window.into(), &mut info) })?;
    Ok(info)
}

/// \[[GetCurrentConsoleFontEx]\] Retrieves extended information about the current console font.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// let info = get_current_console_font_ex(&stdout(), false)?;
/// # Ok(())
/// # })();
/// ```
///
/// [GetCurrentConsoleFontEx]:  https://docs.microsoft.com/en-us/windows/console/getcurrentconsolefontex
pub fn get_current_console_font_ex(console_output: &impl AsConsoleOutputHandle, maximum_window: bool) -> io::Result<CONSOLE_FONT_INFOEX> {
    let mut info : CONSOLE_FONT_INFOEX = unsafe { zeroed() };
    info.cbSize = size_of_val(&info) as _;
    succeeded_to_result(unsafe { GetCurrentConsoleFontEx(console_output.as_raw_handle().cast(), maximum_window.into(), &mut info) })?;
    Ok(info)
}

// TODO: SetCurrentConsoleFontEx
