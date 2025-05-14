use crate::*;

use winapi::shared::minwindef::DWORD;
use winapi::um::wincon::*;

use std::io;



#[doc(alias = "GetConsoleFontSize")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/getconsolefontsize)\]
/// Retrieves the size of the font used by the specified console screen buffer.
///
/// ### Safety
///
/// * $5 says `font` is missing bounds checks or overflows on some version of Windows, Wine, or ReactOS.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// let font = get_current_console_font(&stdout(), false)?;
/// let font_size = unsafe { get_console_font_size(&stdout(), font.font)? };
/// assert_eq!(font_size.x, font.font_size.x);
/// assert_eq!(font_size.y, font.font_size.y);
/// # Ok(())
/// # })();
/// ```
///
pub unsafe fn get_console_font_size(console_output: &impl AsConsoleOutputHandle, nfont: DWORD) -> io::Result<Coord> {
    match unsafe { GetConsoleFontSize(console_output.as_raw_handle().cast(), nfont) } {
        COORD { X: 0, Y: 0 }    => Err(io::Error::last_os_error()),
        coord                   => Ok(coord.into()),
    }
}

#[doc(alias = "GetCurrentConsoleFont")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/getcurrentconsolefont)\]
/// Retrieves information about the current console font.
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
pub fn get_current_console_font(console_output: &impl AsConsoleOutputHandle, maximum_window: bool) -> io::Result<ConsoleFontInfo> {
    let mut info = Default::default();
    succeeded_to_result(unsafe { GetCurrentConsoleFont(console_output.as_raw_handle().cast(), maximum_window.into(), &mut info) })?;
    Ok(info.into())
}

#[doc(alias = "GetCurrentConsoleFontEx")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/getcurrentconsolefontex)\]
/// Retrieves extended information about the current console font.
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
pub fn get_current_console_font_ex(console_output: &impl AsConsoleOutputHandle, maximum_window: bool) -> io::Result<ConsoleFontInfoEx> {
    let mut info : CONSOLE_FONT_INFOEX = Default::default();
    info.cbSize = size_of_val_32_sized(&info);
    succeeded_to_result(unsafe { GetCurrentConsoleFontEx(console_output.as_raw_handle().cast(), maximum_window.into(), &mut info) })?;
    Ok(info.into())
}

// TODO: SetCurrentConsoleFontEx
