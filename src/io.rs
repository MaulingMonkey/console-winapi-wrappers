use crate::*;

use winapi::shared::minwindef::DWORD;
use winapi::um::consoleapi::*;
use winapi::um::wincon::*;

use std::io;

use core::ptr::*;



#[doc(alias = "FillConsoleOutputAttribute")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/fillconsoleoutputattribute)\]
/// Alters `length` cells starting at `coord` to have the same [`Attributes`].
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// assert_eq!(12, fill_console_output_attribute(&mut stdout(), FOREGROUND_GREEN, 12, (0,0))?);
/// # Ok(())
/// # })();
/// ```
///
pub fn fill_console_output_attribute(
    console_output: &mut impl AsConsoleOutputHandle,
    attribute:      impl Into<Attributes>,
    length:         DWORD,
    write_coord:    impl Into<Coord>,
) -> io::Result<DWORD> {
    let mut written = 0;
    succeeded_to_result(unsafe { FillConsoleOutputAttribute(console_output.as_raw_handle().cast(), attribute.into().into(), length, write_coord.into().into(), &mut written) })?;
    Ok(written)
}

#[doc(alias = "FillConsoleOutputCharacter")]
#[doc(alias = "FillConsoleOutputCharacterW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/fillconsoleoutputcharacter)\]
/// Alters `length` cells starting at `coord` to have the same `character`.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// assert_eq!(12, fill_console_output_character(&mut stdout(), 'X', 12, (0,0)).unwrap_or(12));
/// # Ok(())
/// # })();
/// ```
///
pub fn fill_console_output_character(
    console_output: &mut impl AsConsoleOutputHandle,
    character:      char,
    length:         DWORD,
    write_coord:    impl Into<Coord>,
) -> io::Result<DWORD> {
    let character = u16::try_from(character as u32).map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "fill_console_output_character: `character` exceeds maximum codepoint U+FFFF"))?;
    let mut written = 0;
    succeeded_to_result(unsafe { FillConsoleOutputCharacterW(console_output.as_raw_handle().cast(), character, length, write_coord.into().into(), &mut written) })?;
    Ok(written)
}

#[doc(alias = "FlushConsoleInputBuffer")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/flushconsoleinputbuffer)\]
/// Discards everything in the console input buffer.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// flush_console_input_buffer(&mut stdin())?;
/// # Ok(())
/// # })();
/// ```
///
pub fn flush_console_input_buffer(console_input: &mut impl AsConsoleInputHandle) -> io::Result<()> {
    succeeded_to_result(unsafe { FlushConsoleInputBuffer(console_input.as_raw_handle().cast()) })
}

#[doc(alias = "GetConsoleCursorInfo")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/getconsolecursorinfo)\]
/// Retrieves the size and visibility of a screen buffer's cursor.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// let info = get_console_cursor_info(&stdout())?;
/// assert!((1 ..= 100).contains(&info.size));
/// assert!((false ..= true).contains(&info.visible));
/// # Ok(())
/// # })();
/// ```
///
pub fn get_console_cursor_info(console_output: &impl AsConsoleOutputHandle) -> io::Result<ConsoleCursorInfo> {
    let mut info = CONSOLE_CURSOR_INFO::default();
    succeeded_to_result(unsafe { GetConsoleCursorInfo(console_output.as_raw_handle().cast(), &mut info) })?;
    Ok(info.into())
}

#[doc(alias = "GetConsoleDisplayMode")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/getconsoledisplaymode)\]
/// Retrieves the fullscreen state of the current console.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use winapi::shared::minwindef::DWORD;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// let mode : DWORD = get_console_display_mode()?;
/// # Ok(())
/// # })();
/// ```
///
pub fn get_console_display_mode() -> io::Result<DWORD> {
    let mut mode = 0;
    succeeded_to_result(unsafe { GetConsoleDisplayMode(&mut mode) })?;
    Ok(mode)
}

#[doc(alias = "GetConsoleHistoryInfo")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/getconsolehistoryinfo)\]
/// Retrieves the history settings for the calling process's console.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// let info = get_console_history_info()?;
/// # Ok(())
/// # })();
/// ```
///
pub fn get_console_history_info() -> io::Result<CONSOLE_HISTORY_INFO> {
    let mut info = Default::default();
    succeeded_to_result(unsafe { GetConsoleHistoryInfo(&mut info) })?;
    Ok(info)
}

#[doc(alias = "GetConsoleScreenBufferInfo")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/getconsolescreenbufferinfo)\]
/// Retrieves information about the specified console screen buffer.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// let info = get_console_screen_buffer_info(&stdout())?;
/// # Ok(())
/// # })();
/// ```
///
pub fn get_console_screen_buffer_info(console_output: &impl AsConsoleOutputHandle) -> io::Result<ConsoleScreenBufferInfo> {
    let mut info = Default::default();
    succeeded_to_result(unsafe { GetConsoleScreenBufferInfo(console_output.as_raw_handle().cast(), &mut info) })?;
    Ok(info.into())
}

#[doc(alias = "GetConsoleScreenBufferInfoEx")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/getconsolescreenbufferinfoex)\]
/// Retrieves extended information about the specified console screen buffer.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// let info = get_console_screen_buffer_info_ex(&stdout())?;
/// # Ok(())
/// # })();
/// ```
///
pub fn get_console_screen_buffer_info_ex(console_output: &impl AsConsoleOutputHandle) -> io::Result<ConsoleScreenBufferInfoEx> {
    let mut info : CONSOLE_SCREEN_BUFFER_INFOEX = Default::default();
    info.cbSize = size_of_val_32_sized(&info);
    succeeded_to_result(unsafe { GetConsoleScreenBufferInfoEx(console_output.as_raw_handle().cast(), &mut info) })?;
    Ok(info.into())
}

#[doc(alias = "GetConsoleSelectionInfo")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/getconsoleselectioninfo)\]
/// Retrieves information about the current console selection.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// let info = get_console_selection_info()?;
/// # Ok(())
/// # })();
/// ```
///
pub fn get_console_selection_info() -> io::Result<CONSOLE_SELECTION_INFO> {
    let mut info = Default::default();
    succeeded_to_result(unsafe { GetConsoleSelectionInfo(&mut info) })?;
    Ok(info)
}

#[doc(alias = "GetNumberOfConsoleInputEvents")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/getnumberofconsoleinputevents)\]
/// Retrieves the number of unread records in a console's input buffer.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// let events : usize = get_number_of_console_input_events(&stdin())?;
/// # Ok(())
/// # })();
/// ```
///
pub fn get_number_of_console_input_events(console_input: &impl AsConsoleInputHandle) -> io::Result<usize> {
    let mut n = 0;
    succeeded_to_result(unsafe { GetNumberOfConsoleInputEvents(console_input.as_raw_handle().cast(), &mut n) })?;
    Ok(n as _)
}

#[doc(alias = "GetNumberOfConsoleMouseButtons")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/getnumberofconsolemousebuttons)\]
/// Retrieves the number of buttons on the current console's mouse.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// let has_mmb = get_number_of_console_mouse_buttons()? >= 3;
/// # Ok(())
/// # })();
/// ```
///
pub fn get_number_of_console_mouse_buttons() -> io::Result<usize> {
    let mut buttons = 0;
    succeeded_to_result(unsafe { GetNumberOfConsoleMouseButtons(&mut buttons) })?;
    Ok(buttons as _)
}

// GetStdHandle: Prefer AsRawHandle

#[doc(alias = "PeekConsoleInput")]
#[doc(alias = "PeekConsoleInputW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/peekconsoleinput)\]
/// Peek at a console's oldest input records, retaining them in the buffer.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// for record in peek_console_input_with(&stdin(), &mut <[_; 8]>::default())? {
///     // ...
/// }
/// # Ok(())
/// # })();
/// ```
///
pub fn peek_console_input_with<'i>(console_input: &impl AsConsoleInputHandle, buffer: &'i mut [InputRecord]) -> io::Result<&'i [InputRecord]> {
    let length : DWORD = buffer.len().try_into().unwrap_or(!0);
    let mut read = 0;
    succeeded_to_result(unsafe { PeekConsoleInputW(console_input.as_raw_handle().cast(), buffer.as_mut_ptr().cast(), length, &mut read) })?;
    Ok(&buffer[.. read as _])
}

#[doc(alias = "PeekConsoleInput")]
#[doc(alias = "PeekConsoleInputW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/peekconsoleinput)\]
/// Peek at a console's oldest input records, retaining them in the buffer.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// for record in peek_console_input(&stdin())? {
///     // ...
/// }
/// # Ok(())
/// # })();
/// ```
///
pub fn peek_console_input<'i>(console_input: &impl AsConsoleInputHandle) -> io::Result<impl Iterator<Item = InputRecord>> {
    const N : usize = 32;
    let mut buffer = [InputRecord::default(); N];
    let mut read = 0;
    succeeded_to_result(unsafe { PeekConsoleInputW(console_input.as_raw_handle().cast(), buffer.as_mut_ptr().cast(), N as _, &mut read) })?;
    #[allow(deprecated)] // avoid bumping MSRV
    Ok(std::array::IntoIter::new(buffer).take(read as _))
}

#[doc(alias = "PeekConsoleInput")]
#[doc(alias = "PeekConsoleInputW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/peekconsoleinput)\]
/// Peek at a console's oldest input record, retaining it in the buffer.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// if let Some(record) = peek_console_input_one(&stdin())? {
///     // ...
/// }
/// # Ok(())
/// # })();
/// ```
///
pub fn peek_console_input_one(console_input: &impl AsConsoleInputHandle) -> io::Result<Option<InputRecord>> {
    let mut record = InputRecord::default();
    let empty = peek_console_input_with(console_input, std::slice::from_mut(&mut record))?.is_empty();
    Ok(if empty { None } else { Some(record) })
}

#[doc(alias = "ReadConsole")]
#[doc(alias = "ReadConsoleW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/readconsole)\]
/// Reads and removes character input from a console's input buffer.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// let mut buffer = [0u16; 1024];
/// let input : &[u16] = read_console(&mut stdin(), &mut buffer, None)?;
/// # Ok(())
/// # })();
/// ```
///
pub fn read_console<'i>(console_input: &mut impl AsConsoleInputHandle, buffer: &'i mut [u16], input_control: Option<&CONSOLE_READCONSOLE_CONTROL>) -> io::Result<&'i [u16]> {
    let input_control : *const CONSOLE_READCONSOLE_CONTROL = match input_control {
        None => null(),
        Some(ctrl) => ctrl,
    };
    let length : DWORD = buffer.len().try_into().unwrap_or(!0);
    let mut read = 0;
    succeeded_to_result(unsafe { ReadConsoleW(console_input.as_raw_handle().cast(), buffer.as_mut_ptr().cast(), length, &mut read, input_control as *mut _) })?;
    Ok(&buffer[.. read as _])
}

#[doc(alias = "ReadConsoleInput")]
#[doc(alias = "ReadConsoleInputW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/readconsoleinput)\]
/// Reads and removes the oldest records from a console's input buffer.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// for record in read_console_input_with(&mut stdin(), &mut <[_; 8]>::default())? {
///     // ...
/// }
/// # Ok(())
/// # })();
/// ```
///
pub fn read_console_input_with<'i>(console_input: &mut impl AsConsoleInputHandle, buffer: &'i mut [InputRecord]) -> io::Result<&'i [InputRecord]> {
    let length : DWORD = buffer.len().try_into().unwrap_or(!0);
    let mut read = 0;
    succeeded_to_result(unsafe { ReadConsoleInputW(console_input.as_raw_handle().cast(), buffer.as_mut_ptr().cast(), length, &mut read) })?;
    Ok(&buffer[.. read as _])
}

#[doc(alias = "ReadConsoleInput")]
#[doc(alias = "ReadConsoleInputW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/readconsoleinput)\]
/// Reads and removes the oldest records from a console's input buffer.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// for record in read_console_input(&mut stdin())? {
///     // ...
/// }
/// # Ok(())
/// # })();
/// ```
///
pub fn read_console_input<'i>(console_input: &mut impl AsConsoleInputHandle) -> io::Result<impl Iterator<Item = InputRecord>> {
    const N : usize = 32;
    let mut buffer = [InputRecord::default(); N];
    let mut read = 0;
    succeeded_to_result(unsafe { ReadConsoleInputW(console_input.as_raw_handle().cast(), buffer.as_mut_ptr().cast(), N as _, &mut read) })?;
    #[allow(deprecated)] // avoid bumping MSRV
    Ok(std::array::IntoIter::new(buffer).take(read as _))
}

#[doc(alias = "ReadConsoleInput")]
#[doc(alias = "ReadConsoleInputW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/readconsoleinput)\]
/// Reads and removes the oldest record from a console's input buffer.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// let record = read_console_input_one(&mut stdin())?;
/// # Ok(())
/// # })();
/// ```
///
pub fn read_console_input_one<'i>(console_input: &mut impl AsConsoleInputHandle) -> io::Result<InputRecord> {
    let mut record = InputRecord::default();
    let mut read = 0;
    succeeded_to_result(unsafe { ReadConsoleInputW(console_input.as_raw_handle().cast(), core::ptr::from_mut(&mut record).cast(), 1, &mut read) })?;
    debug_assert_eq!(read, 1);
    Ok(record)
}

#[doc(alias = "ReadConsoleOutput")]
#[doc(alias = "ReadConsoleOutputW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/readconsoleoutput)\]
/// Reads a rectangular block of characters and [Attributes] from a screen.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// use winapi::um::wincontypes::{SMALL_RECT};
/// let mut buffer = [CharInfo::default(); 80 * 25];
/// let mut region = SMALL_RECT { Left: 0, Right: 80, Top: 0, Bottom: 25 };
/// read_console_output(&stdout(), &mut buffer, (80,25), (0,0), &mut region)?;
/// # Ok(())
/// # })();
/// ```
///
pub fn read_console_output(console_output: &impl AsConsoleOutputHandle, buffer: &mut [CharInfo], buffer_size: impl Into<Coord>, buffer_coord: impl Into<Coord>, read_region: &mut SMALL_RECT) -> io::Result<()> {
    let console_output = console_output.as_raw_handle().cast();
    let buffer_size  : COORD = buffer_size.into().into();
    let buffer_coord : COORD = buffer_coord.into().into();

    // Bounds checking
    if buffer_coord.X > buffer_size.X { return Err(io::Error::new(io::ErrorKind::InvalidInput, "read_console_output(): buffer_coord.X > buffer_size.X")); }
    if buffer_coord.Y > buffer_size.Y { return Err(io::Error::new(io::ErrorKind::InvalidInput, "read_console_output(): buffer_coord.Y > buffer_size.Y")); }
    let buffer_w = usize::try_from(buffer_size.X).map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "read_console_output(): buffer_size.X is out of SHORT bounds"))?;
    let buffer_h = usize::try_from(buffer_size.Y).map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "read_console_output(): buffer_size.Y is out of SHORT bounds"))?;
    let buffer_size_total = buffer_w.checked_mul(buffer_h).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "read_console_output(): buffer_size.X * Y is out of usize bounds"))?;
    if buffer_size_total > buffer.len() { return Err(io::Error::new(io::ErrorKind::InvalidInput, "read_console_output(): buffer_size is larger than buffer")); }
    // TODO: read_region bounds checking?

    succeeded_to_result(unsafe { ReadConsoleOutputW(console_output, buffer.as_mut_ptr().cast(), buffer_size, buffer_coord, read_region) })
}

#[doc(alias = "ReadConsoleOutputAttribute")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/readconsoleoutputattribute)\]
/// Reads a linear line of [Attributes] from a console screen buffer.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// let mut buffer = [Attributes::default(); 80];
/// read_console_output_attribute(&stdout(), &mut buffer, (0,0))?;
/// # Ok(())
/// # })();
/// ```
///
pub fn read_console_output_attribute<'a>(console_output: &impl AsConsoleOutputHandle, attributes: &'a mut [Attributes], read_coord: impl Into<Coord>) -> io::Result<&'a [Attributes]> {
    let console_output = console_output.as_raw_handle().cast();
    let length : DWORD = attributes.len().try_into().unwrap_or(!0);
    let read_coord = read_coord.into().into();

    let mut read = 0;
    succeeded_to_result(unsafe { ReadConsoleOutputAttribute(console_output, attributes.as_mut_ptr().cast(), length, read_coord, &mut read) })?;
    Ok(&attributes[.. read as _])
}

#[doc(alias = "ReadConsoleOutputCharacter")]
#[doc(alias = "ReadConsoleOutputCharacterW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/readconsoleoutputcharacter)\]
/// Reads a linear line of characters from a console screen buffer.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// let mut buffer = [0u16; 80];
/// read_console_output_character(&stdout(), &mut buffer, (0,0))?;
/// # Ok(())
/// # })();
/// ```
///
pub fn read_console_output_character<'a>(console_output: &impl AsConsoleOutputHandle, characters: &'a mut [u16], read_coord: impl Into<Coord>) -> io::Result<&'a [u16]> {
    let console_output = console_output.as_raw_handle().cast();
    let length : DWORD = characters.len().try_into().unwrap_or(!0);
    let read_coord = read_coord.into().into();

    let mut read = 0;
    succeeded_to_result(unsafe { ReadConsoleOutputCharacterW(console_output, characters.as_mut_ptr(), length, read_coord, &mut read) })?;
    Ok(&characters[.. read as _])
}

// TODO: ScrollConsoleScreenBuffer

#[doc(alias = "SetConsoleActiveScreenBuffer")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/setconsoleactivescreenbuffer)\]
/// Sets the specified screen buffer to be displayed on the current console.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// set_console_active_screen_buffer(&stdout())?;
/// set_console_active_screen_buffer(&stdout())?;
/// # Ok(())
/// # })();
/// ```
///
pub fn set_console_active_screen_buffer(console_output: &impl AsConsoleOutputHandle) -> io::Result<()> {
    succeeded_to_result(unsafe { SetConsoleActiveScreenBuffer(console_output.as_raw_handle().cast()) })
}

#[doc(alias = "SetConsoleCursorInfo")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/setconsolecursorinfo)\]
/// Sets the size and visibility of the cursor for a console screen buffer.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// set_console_cursor_info(&mut stdout(), ConsoleCursorInfo::new(100, true))?;
/// # Ok(())
/// # })();
/// ```
///
pub fn set_console_cursor_info(console_output: &mut impl AsConsoleOutputHandle, info: impl Into<ConsoleCursorInfo>) -> io::Result<()> {
    let mut info : CONSOLE_CURSOR_INFO = info.into().into();

    // Sanitize
    if info.dwSize < 1   { info.dwSize = 1;   }
    if info.dwSize > 100 { info.dwSize = 100; }
    info.bVisible = (info.bVisible != 0) as _;

    succeeded_to_result(unsafe { SetConsoleCursorInfo(console_output.as_raw_handle().cast(), &info) })
}

#[doc(alias = "SetConsoleCursorPosition")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/setconsolecursorposition)\]
/// Sets the cursor position in the specified console screen buffer.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// set_console_cursor_position(&mut stdout(), (0,0))?;
/// # Ok(())
/// # })();
/// ```
///
pub fn set_console_cursor_position(console_output: &mut impl AsConsoleOutputHandle, cursor_position: impl Into<Coord>) -> io::Result<()> {
    succeeded_to_result(unsafe { SetConsoleCursorPosition(console_output.as_raw_handle().cast(), cursor_position.into().into()) })
}

// TODO: SetConsoleDisplayMode
// TODO: SetConsoleHistoryInfo
// TODO: SetConsoleScreenBufferInfoEx
// TODO: SetConsoleScreenBufferInfo

#[doc(alias = "SetConsoleScreenBufferSize")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/setconsolescreenbuffersize)\]
/// Changes the size of the specified console screen buffer.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// set_console_screen_buffer_size(&mut stdout(), (80, 25))?;
/// set_console_screen_buffer_size(&mut stdout(), [80, 25])?;
/// set_console_screen_buffer_size(&mut stdout(), Coord { x: 80, y: 25 })?;
/// use winapi::um::wincontypes::COORD;
/// set_console_screen_buffer_size(&mut stdout(), COORD { X: 80, Y: 25 })?;
/// # Ok(())
/// # })();
/// ```
///
pub fn set_console_screen_buffer_size(console_output: &mut impl AsConsoleOutputHandle, size: impl Into<Coord>) -> io::Result<()> {
    succeeded_to_result(unsafe { SetConsoleScreenBufferSize(console_output.as_raw_handle().cast(), size.into().into()) })
}

#[doc(alias = "SetConsoleTextAttribute")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/setconsoletextattribute)\]
/// Sets the default [Attributes] to use for future writes, or echoed reads.
///
/// Writing fns affected:
/// -   [WriteFile](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-writefile)
/// -   [WriteConsole](https://learn.microsoft.com/en-us/windows/console/writeconsole)
///
/// Echoing fns affected:
/// -   [ReadFile](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile)
/// -   [ReadConsole](https://learn.microsoft.com/en-us/windows/console/readconsole)
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// set_console_text_attribute(&mut stdout(), FOREGROUND_GREEN)?;
/// # Ok(())
/// # })();
/// ```
///
pub fn set_console_text_attribute(console_output: &mut impl AsConsoleOutputHandle, attributes: impl Into<Attributes>) -> io::Result<()> {
    succeeded_to_result(unsafe { SetConsoleTextAttribute(console_output.as_raw_handle().cast(), attributes.into().into()) })
}

// TODO: SetConsoleWindowInfo
// TODO: SetStdHandle?

#[doc(alias = "WriteConsole")]
#[doc(alias = "WriteConsoleW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/writeconsole)\]
/// Writes a string to a screen buffer, beginning at and advancing the cursor.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// write_console(&mut stdout(), wchar::wch!("Hello, world!"), ())?;
/// # Ok(())
/// # })();
/// ```
///
pub fn write_console(console_output: &mut impl AsConsoleOutputHandle, buffer: &[u16], _reserved: impl Reserved) -> io::Result<usize> {
    let length = buffer.len().try_into().unwrap_or(!0);
    let mut written = 0;
    succeeded_to_result(unsafe { WriteConsoleW(console_output.as_raw_handle().cast(), buffer.as_ptr().cast(), length, &mut written, null_mut()) })?;
    Ok(written as _)
}

#[doc(alias = "WriteConsoleInput")]
#[doc(alias = "WriteConsoleInputW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/writeconsoleinput)\]
/// Writes data directly to the console input buffer.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> { unsafe {
/// use winapi::um::winuser::VK_SPACE;
///
/// let input = KeyEventRecord {
///     key_down:           true.into(),
///     repeat_count:       0,
///     virtual_key_code:   VK_SPACE as _,
///     virtual_scan_code:  b' '.into(),
///     char:               u16::from(b' ').into(),
///     control_key_state:  0,
/// };
///
/// write_console_input(&mut stdin(), &[input.into()])?;
/// # Ok(())
/// # }})();
/// ```
///
pub fn write_console_input(console_input: &mut impl AsConsoleInputHandle, buffer: &[InputRecord]) -> io::Result<usize> {
    let length = buffer.len().try_into().unwrap_or(!0);
    let mut written = 0;
    succeeded_to_result(unsafe { WriteConsoleInputW(console_input.as_raw_handle().cast(), buffer.as_ptr().cast(), length, &mut written) })?;
    Ok(written as _ )
}

#[doc(alias = "WriteConsoleOutput")]
#[doc(alias = "WriteConsoleOutputW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/writeconsoleoutput)\]
/// Copies a rectangular region of characters and [Attributes] to screen buffers.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// use winapi::um::{wincontypes::*, winuser::*};
///
/// let output = [CharInfo::new(u16::from(b' '), Attributes::FOREGROUND_GREEN)];
/// let mut region = SMALL_RECT { Left: 10, Right: 11, Top: 20, Bottom: 21 };
///
/// write_console_output(&mut stdout(), &output, (1,1), (0, 0), &mut region)?;
/// # Ok(())
/// # })();
/// ```
///
pub fn write_console_output(console_output: &mut impl AsConsoleOutputHandle, buffer: &[CharInfo], buffer_size: impl Into<Coord>, buffer_coord: impl Into<Coord>, write_region: &mut SMALL_RECT) -> io::Result<()> {
    let console_output = console_output.as_raw_handle().cast();
    let buffer_size  : COORD = buffer_size.into().into();
    let buffer_coord : COORD = buffer_coord.into().into();

    // Bounds checking
    if buffer_coord.X > buffer_size.X { return Err(io::Error::new(io::ErrorKind::InvalidInput, "write_console_output(): buffer_coord.X > buffer_size.X")); }
    if buffer_coord.Y > buffer_size.Y { return Err(io::Error::new(io::ErrorKind::InvalidInput, "write_console_output(): buffer_coord.Y > buffer_size.Y")); }
    let buffer_w = usize::try_from(buffer_size.X).map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "write_console_output(): buffer_size.X is out of SHORT bounds"))?;
    let buffer_h = usize::try_from(buffer_size.Y).map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "write_console_output(): buffer_size.Y is out of SHORT bounds"))?;
    let buffer_size_total = buffer_w.checked_mul(buffer_h).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "write_console_output(): buffer_size.X * Y is out of usize bounds"))?;
    if buffer_size_total > buffer.len() { return Err(io::Error::new(io::ErrorKind::InvalidInput, "write_console_output(): buffer_size is larger than buffer")); }
    // TODO: write_region bounds checking?

    succeeded_to_result(unsafe { WriteConsoleOutputW(console_output, buffer.as_ptr().cast(), buffer_size, buffer_coord, write_region) })
}

#[doc(alias = "WriteConsoleOutputAttribute")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/writeconsoleoutputattribute)\]
/// Copies a line of [Attributes] to a screen buffer, starting at `write_coord`.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// write_console_output_attribute(&mut stdout(), &[FOREGROUND_GREEN], (1,2))?;
/// # Ok(())
/// # })();
/// ```
///
pub fn write_console_output_attribute(console_output: &mut impl AsConsoleOutputHandle, attributes: &[Attributes], write_coord: impl Into<Coord>) -> io::Result<usize> {
    let length = attributes.len().try_into().unwrap_or(!0);
    let mut written = 0;
    succeeded_to_result(unsafe { WriteConsoleOutputAttribute(console_output.as_raw_handle().cast(), attributes.as_ptr().cast(), length, write_coord.into().into(), &mut written) })?;
    Ok(written as _)
}

#[doc(alias = "WriteConsoleOutputCharacter")]
#[doc(alias = "WriteConsoleOutputCharacterW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/writeconsoleoutputcharacter)\]
/// Copies a string of wchar_t to a screen buffer, starting at `write_coord`.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// write_console_output_character(&mut stdout(), &[' ' as u16], (1,2))?;
/// # Ok(())
/// # })();
/// ```
///
pub fn write_console_output_character(console_output: &mut impl AsConsoleOutputHandle, characters: &[u16], write_coord: impl Into<Coord>) -> io::Result<usize> {
    let length = characters.len().try_into().unwrap_or(!0);
    let mut written = 0;
    succeeded_to_result(unsafe { WriteConsoleOutputCharacterW(console_output.as_raw_handle().cast(), characters.as_ptr(), length, write_coord.into().into(), &mut written) })?;
    Ok(written as _)
}
