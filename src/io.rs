use crate::*;

use winapi::shared::minwindef::DWORD;
use winapi::um::consoleapi::*;
use winapi::um::wincon::*;

use std::convert::*;
use std::io;
use std::mem::{size_of_val, zeroed};
use std::ptr::*;



/// \[[FillConsoleOutputAttribute]\] Sets the character attributes for a specified number of character cells, beginning at the specified coordinates in a screen buffer.
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
/// [FillConsoleOutputAttribute]:   https://docs.microsoft.com/en-us/windows/console/fillconsoleoutputattribute
pub fn fill_console_output_attribute(
    console_output: &mut impl AsConsoleOutputHandle,
    attribute:      impl Into<Attributes>,
    length:         DWORD,
    write_coord:    impl IntoCoord,
) -> io::Result<DWORD> {
    let mut written = 0;
    succeeded_to_result(unsafe { FillConsoleOutputAttribute(console_output.as_raw_handle().cast(), attribute.into().into(), length, write_coord.into_coord(), &mut written) })?;
    Ok(written)
}

/// \[[FillConsoleOutputCharacterW]\] Sets the character characters for a specified number of character cells, beginning at the specified coordinates in a screen buffer.
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
/// [FillConsoleOutputCharacterW]:  https://docs.microsoft.com/en-us/windows/console/fillconsoleoutputcharacter
pub fn fill_console_output_character(
    console_output: &mut impl AsConsoleOutputHandle,
    character:      char,
    length:         DWORD,
    write_coord:    impl IntoCoord,
) -> io::Result<DWORD> {
    let character = u16::try_from(character as u32).map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "fill_console_output_character: `character` exceeds maximum codepoint U+FFFF"))?;
    let mut written = 0;
    succeeded_to_result(unsafe { FillConsoleOutputCharacterW(console_output.as_raw_handle().cast(), character, length, write_coord.into_coord(), &mut written) })?;
    Ok(written)
}

/// \[[FlushConsoleInputBuffer]\] Flushes the console input buffer. All input records currently in the input buffer are discarded.
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
/// [FlushConsoleInputBuffer]:  https://docs.microsoft.com/en-us/windows/console/flushconsoleinputbuffer
pub fn flush_console_input_buffer(console_input: &mut impl AsConsoleInputHandle) -> io::Result<()> {
    succeeded_to_result(unsafe { FlushConsoleInputBuffer(console_input.as_raw_handle().cast()) })
}

/// \[[GetConsoleCursorInfo]\] Retrieves information about the size and visibility of the cursor for the specified console screen buffer.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// let info = get_console_cursor_info(&stdout())?;
/// assert!((1 ..= 100).contains(&info.dwSize));
/// assert!((0 ..= 1).contains(&info.bVisible));
/// # Ok(())
/// # })();
/// ```
///
/// [GetConsoleCursorInfo]: https://docs.microsoft.com/en-us/windows/console/getconsolecursorinfo
pub fn get_console_cursor_info(console_output: &impl AsConsoleOutputHandle) -> io::Result<CONSOLE_CURSOR_INFO> {
    let mut info : CONSOLE_CURSOR_INFO = unsafe { zeroed() };
    succeeded_to_result(unsafe { GetConsoleCursorInfo(console_output.as_raw_handle().cast(), &mut info) })?;
    Ok(info)
}

/// \[[GetConsoleDisplayMode]\] Retrieves the display mode of the current console.
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
/// [GetConsoleDisplayMode]:    https://docs.microsoft.com/en-us/windows/console/getconsoledisplaymode
pub fn get_console_display_mode() -> io::Result<DWORD> {
    let mut mode = 0;
    succeeded_to_result(unsafe { GetConsoleDisplayMode(&mut mode) })?;
    Ok(mode)
}

/// \[[GetConsoleHistoryInfo]\] Retrieves the history settings for the calling process's console.
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
/// [GetConsoleHistoryInfo]:    https://docs.microsoft.com/en-us/windows/console/getconsolehistoryinfo
pub fn get_console_history_info() -> io::Result<CONSOLE_HISTORY_INFO> {
    let mut info : CONSOLE_HISTORY_INFO = unsafe { zeroed() };
    succeeded_to_result(unsafe { GetConsoleHistoryInfo(&mut info) })?;
    Ok(info)
}

/// \[[GetConsoleScreenBufferInfo]\] Retrieves information about the specified console screen buffer.
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
/// [GetConsoleScreenBufferInfo]:   https://docs.microsoft.com/en-us/windows/console/getconsolescreenbufferinfo
pub fn get_console_screen_buffer_info(console_output: &impl AsConsoleOutputHandle) -> io::Result<CONSOLE_SCREEN_BUFFER_INFO> {
    let mut info : CONSOLE_SCREEN_BUFFER_INFO = unsafe { zeroed() };
    succeeded_to_result(unsafe { GetConsoleScreenBufferInfo(console_output.as_raw_handle().cast(), &mut info) })?;
    Ok(info)
}

/// \[[GetConsoleScreenBufferInfoEx]\] Retrieves extended information about the specified console screen buffer.
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
/// [GetConsoleScreenBufferInfoEx]: https://docs.microsoft.com/en-us/windows/console/getconsolescreenbufferinfoex
pub fn get_console_screen_buffer_info_ex(console_output: &impl AsConsoleOutputHandle) -> io::Result<CONSOLE_SCREEN_BUFFER_INFOEX> {
    let mut info : CONSOLE_SCREEN_BUFFER_INFOEX = unsafe { zeroed() };
    info.cbSize = size_of_val(&info) as _;
    succeeded_to_result(unsafe { GetConsoleScreenBufferInfoEx(console_output.as_raw_handle().cast(), &mut info) })?;
    Ok(info)
}

/// \[[GetConsoleSelectionInfo]\] Retrieves information about the current console selection.
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
/// [GetConsoleSelectionInfo]:  https://docs.microsoft.com/en-us/windows/console/getconsoleselectioninfo
pub fn get_console_selection_info() -> io::Result<CONSOLE_SELECTION_INFO> {
    let mut info : CONSOLE_SELECTION_INFO = unsafe { zeroed() };
    succeeded_to_result(unsafe { GetConsoleSelectionInfo(&mut info) })?;
    Ok(info)
}

/// \[[GetNumberOfConsoleInputEvents]\] Retrieves the number of unread input records in the console's input buffer.
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
/// [GetNumberOfConsoleInputEvents]:    https://docs.microsoft.com/en-us/windows/console/getnumberofconsoleinputevents
pub fn get_number_of_console_input_events(console_input: &impl AsConsoleInputHandle) -> io::Result<usize> {
    let mut n = 0;
    succeeded_to_result(unsafe { GetNumberOfConsoleInputEvents(console_input.as_raw_handle().cast(), &mut n) })?;
    Ok(n as _)
}

/// \[[GetNumberOfConsoleMouseButtons]\] Retrieves the number of buttons on the mouse used by the current console.
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
/// [GetNumberOfConsoleMouseButtons]:   https://docs.microsoft.com/en-us/windows/console/getnumberofconsolemousebuttons
pub fn get_number_of_console_mouse_buttons() -> io::Result<usize> {
    let mut buttons = 0;
    succeeded_to_result(unsafe { GetNumberOfConsoleMouseButtons(&mut buttons) })?;
    Ok(buttons as _)
}

// GetStdHandle: Prefer AsRawHandle

/// \[[PeekConsoleInput]\] Reads data from the specified console input buffer without removing it from the buffer.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// for record in peek_console_input(&stdin(), &mut <[_; 8]>::default())? {
///     // ...
/// }
/// # Ok(())
/// # })();
/// ```
///
/// [PeekConsoleInput]: https://docs.microsoft.com/en-us/windows/console/peekconsoleinput
pub fn peek_console_input<'i>(console_input: &impl AsConsoleInputHandle, buffer: &'i mut [INPUT_RECORD]) -> io::Result<&'i [INPUT_RECORD]> {
    let length : DWORD = buffer.len().try_into().unwrap_or(!0);
    let mut read = 0;
    succeeded_to_result(unsafe { PeekConsoleInputW(console_input.as_raw_handle().cast(), buffer.as_mut_ptr(), length, &mut read) })?;
    Ok(&buffer[.. read as _])
}

/// \[[PeekConsoleInput]\] Reads data from the specified console input buffer without removing it from the buffer.
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
/// [PeekConsoleInput]: https://docs.microsoft.com/en-us/windows/console/peekconsoleinput
pub fn peek_console_input_one(console_input: &impl AsConsoleInputHandle) -> io::Result<Option<INPUT_RECORD>> {
    let mut record : INPUT_RECORD = unsafe { zeroed() };
    let empty = peek_console_input(console_input, std::slice::from_mut(&mut record))?.is_empty();
    Ok(if empty { None } else { Some(record) })
}

/// \[[ReadConsole]\] Reads character input from the console input buffer and removes it from the buffer.
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
/// [ReadConsole]:  https://docs.microsoft.com/en-us/windows/console/readconsole
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

/// \[[ReadConsoleInput]\] Reads data from a console input buffer and removes it from the buffer.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// for record in read_console_input(&mut stdin(), &mut <[_; 8]>::default())? {
///     // ...
/// }
/// # Ok(())
/// # })();
/// ```
///
/// [ReadConsoleInput]: https://docs.microsoft.com/en-us/windows/console/readconsoleinput
pub fn read_console_input<'i>(console_input: &mut impl AsConsoleInputHandle, buffer: &'i mut [INPUT_RECORD]) -> io::Result<&'i [INPUT_RECORD]> {
    let length : DWORD = buffer.len().try_into().unwrap_or(!0);
    let mut read = 0;
    succeeded_to_result(unsafe { ReadConsoleInputW(console_input.as_raw_handle().cast(), buffer.as_mut_ptr(), length, &mut read) })?;
    Ok(&buffer[.. read as _])
}

/// \[[ReadConsoleOutputW]\] Reads character and color attribute data from a rectangular block of character cells in a console screen buffer, and the function writes the data to a rectangular block at a specified location in the destination buffer.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// use winapi::um::wincontypes::{CHAR_INFO, SMALL_RECT};
/// let mut buffer = [CHAR_INFO::default(); 80 * 25];
/// let mut region = SMALL_RECT { Left: 0, Right: 80, Top: 0, Bottom: 25 };
/// read_console_output(&mut stdout(), &mut buffer, (80,25), (0,0), &mut region)?;
/// # Ok(())
/// # })();
/// ```
///
/// [ReadConsoleOutputW]:   https://docs.microsoft.com/en-us/windows/console/readconsoleoutput
pub fn read_console_output(console_output: &mut impl AsConsoleOutputHandle, buffer: &mut [CHAR_INFO], buffer_size: impl IntoCoord, buffer_coord: impl IntoCoord, read_region: &mut SMALL_RECT) -> io::Result<()> {
    let console_output = console_output.as_raw_handle().cast();
    let buffer_size = buffer_size.into_coord();
    let buffer_coord = buffer_coord.into_coord();

    // Bounds checking
    if buffer_coord.X > buffer_size.X { return Err(io::Error::new(io::ErrorKind::InvalidInput, "read_console_output(): buffer_coord.X > buffer_size.X")); }
    if buffer_coord.Y > buffer_size.Y { return Err(io::Error::new(io::ErrorKind::InvalidInput, "read_console_output(): buffer_coord.Y > buffer_size.Y")); }
    let buffer_w = usize::try_from(buffer_size.X).map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "read_console_output(): buffer_size.X is out of SHORT bounds"))?;
    let buffer_h = usize::try_from(buffer_size.Y).map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "read_console_output(): buffer_size.Y is out of SHORT bounds"))?;
    let buffer_size_total = buffer_w.checked_mul(buffer_h).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "read_console_output(): buffer_size.X * Y is out of usize bounds"))?;
    if buffer_size_total > buffer.len() { return Err(io::Error::new(io::ErrorKind::InvalidInput, "read_console_output(): buffer_size is larger than buffer")); }
    // TODO: read_region bounds checking?

    succeeded_to_result(unsafe { ReadConsoleOutputW(console_output, buffer.as_mut_ptr(), buffer_size, buffer_coord, read_region) })
}

/// \[[ReadConsoleOutputAttribute]\] Copies a specified number of character attributes from consecutive cells of a console screen buffer, beginning at a specified location.
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
/// [ReadConsoleOutputAttribute]:   https://docs.microsoft.com/en-us/windows/console/readconsoleoutputattribute
pub fn read_console_output_attribute<'a>(console_output: &impl AsConsoleOutputHandle, attributes: &'a mut [Attributes], read_coord: impl IntoCoord) -> io::Result<&'a [Attributes]> {
    let console_output = console_output.as_raw_handle().cast();
    let length : DWORD = attributes.len().try_into().unwrap_or(!0);
    let read_coord = read_coord.into_coord();

    let mut read = 0;
    succeeded_to_result(unsafe { ReadConsoleOutputAttribute(console_output, attributes.as_mut_ptr().cast(), length, read_coord, &mut read) })?;
    Ok(&attributes[.. read as _])
}

/// \[[ReadConsoleOutputCharacter]\] Copies a specified number of character attributes from consecutive cells of a console screen buffer, beginning at a specified location.
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
/// [ReadConsoleOutputCharacter]:   https://docs.microsoft.com/en-us/windows/console/readconsoleoutputcharacter
pub fn read_console_output_character<'a>(console_output: &impl AsConsoleOutputHandle, characters: &'a mut [u16], read_coord: impl IntoCoord) -> io::Result<&'a [u16]> {
    let console_output = console_output.as_raw_handle().cast();
    let length : DWORD = characters.len().try_into().unwrap_or(!0);
    let read_coord = read_coord.into_coord();

    let mut read = 0;
    succeeded_to_result(unsafe { ReadConsoleOutputCharacterW(console_output, characters.as_mut_ptr(), length, read_coord, &mut read) })?;
    Ok(&characters[.. read as _])
}

// TODO: ScrollConsoleScreenBuffer

/// \[[SetConsoleActiveScreenBuffer]\] Sets the specified screen buffer to be the currently displayed console screen buffer.
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
/// [SetConsoleActiveScreenBuffer]: https://docs.microsoft.com/en-us/windows/console/setconsoleactivescreenbuffer
pub fn set_console_active_screen_buffer(console_output: &impl AsConsoleOutputHandle) -> io::Result<()> {
    succeeded_to_result(unsafe { SetConsoleActiveScreenBuffer(console_output.as_raw_handle().cast()) })
}

/// \[[SetConsoleCursorInfo]\] Sets the size and visibility of the cursor for the specified console screen buffer.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// use winapi::um::wincon::CONSOLE_CURSOR_INFO;
/// set_console_cursor_info(&mut stdout(), CONSOLE_CURSOR_INFO { dwSize: 100, bVisible: 1 })?;
/// # Ok(())
/// # })();
/// ```
///
/// [SetConsoleCursorInfo]: https://docs.microsoft.com/en-us/windows/console/setconsolecursorinfo
pub fn set_console_cursor_info(console_output: &mut impl AsConsoleOutputHandle, info: impl Into<CONSOLE_CURSOR_INFO>) -> io::Result<()> {
    let mut info = info.into();

    // Sanitize
    if info.dwSize < 1   { info.dwSize = 1;   }
    if info.dwSize > 100 { info.dwSize = 100; }
    info.bVisible = (info.bVisible != 0) as _;

    succeeded_to_result(unsafe { SetConsoleCursorInfo(console_output.as_raw_handle().cast(), &info) })
}

/// \[[SetConsoleCursorPosition]\] Sets the cursor position in the specified console screen buffer.
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
/// [SetConsoleCursorPosition]: https://docs.microsoft.com/en-us/windows/console/setconsolecursorposition
pub fn set_console_cursor_position(console_output: &mut impl AsConsoleOutputHandle, cursor_position: impl IntoCoord) -> io::Result<()> {
    succeeded_to_result(unsafe { SetConsoleCursorPosition(console_output.as_raw_handle().cast(), cursor_position.into_coord()) })
}

// TODO: SetConsoleDisplayMode
// TODO: SetConsoleHistoryInfo
// TODO: SetConsoleScreenBufferInfoEx
// TODO: SetConsoleScreenBufferInfo

/// \[[SetConsoleScreenBufferSize]\] Changes the size of the specified console screen buffer.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// set_console_screen_buffer_size(&mut stdout(), (80, 25))?;
/// set_console_screen_buffer_size(&mut stdout(), [80, 25])?;
/// use winapi::um::wincontypes::COORD;
/// set_console_screen_buffer_size(&mut stdout(), COORD { X: 80, Y: 25 })?;
/// # Ok(())
/// # })();
/// ```
///
/// [SetConsoleScreenBufferSize]:   https://docs.microsoft.com/en-us/windows/console/setconsolescreenbuffersize
pub fn set_console_screen_buffer_size(console_output: &mut impl AsConsoleOutputHandle, size: impl IntoCoord) -> io::Result<()> {
    succeeded_to_result(unsafe { SetConsoleScreenBufferSize(console_output.as_raw_handle().cast(), size.into_coord()) })
}

/// \[[SetConsoleTextAttribute]\] Sets the attributes of characters written to the console screen buffer by the [WriteFile] or [WriteConsole] function, or echoed by the [ReadFile] or [ReadConsole] function. This function affects text written after the function call.
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
/// [SetConsoleTextAttribute]:  https://docs.microsoft.com/en-us/windows/console/setconsoletextattribute
/// [WriteFile]:                https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-writefile
/// [WriteConsole]:             https://docs.microsoft.com/en-us/windows/console/writeconsole
/// [ReadFile]:                 https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile
/// [ReadConsole]:              https://docs.microsoft.com/en-us/windows/console/readconsole
pub fn set_console_text_attribute(console_output: &mut impl AsConsoleOutputHandle, attributes: impl Into<Attributes>) -> io::Result<()> {
    succeeded_to_result(unsafe { SetConsoleTextAttribute(console_output.as_raw_handle().cast(), attributes.into().into()) })
}

// TODO: SetConsoleWindowInfo
// TODO: SetStdHandle?

/// \[[WriteConsoleW]\] Writes a character string to a console screen buffer beginning at the current cursor location.
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
/// [WriteConsoleW]: https://docs.microsoft.com/en-us/windows/console/writeconsole
pub fn write_console(console_output: &mut impl AsConsoleOutputHandle, buffer: &[u16], _reserved: impl Reserved) -> io::Result<usize> {
    let length = buffer.len().try_into().unwrap_or(!0);
    let mut written = 0;
    succeeded_to_result(unsafe { WriteConsoleW(console_output.as_raw_handle().cast(), buffer.as_ptr().cast(), length, &mut written, null_mut()) })?;
    Ok(written as _)
}

/// \[[WriteConsoleInput]\] Writes data directly to the console input buffer.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> { unsafe {
/// use winapi::um::{wincontypes::*, winuser::*};
///
/// let mut input = INPUT_RECORD::default();
/// input.EventType = KEY_EVENT;
/// *input.Event.KeyEvent_mut() = KEY_EVENT_RECORD {
///     bKeyDown: 1, wRepeatCount: 0, wVirtualKeyCode: VK_SPACE as _,
///     wVirtualScanCode: b' ' as _, .. Default::default()
/// };
/// *input.Event.KeyEvent_mut().uChar.UnicodeChar_mut() = ' ' as u32 as u16;
///
/// write_console_input(&mut stdin(), &[input])?;
/// # Ok(())
/// # }})();
/// ```
///
/// [WriteConsoleInput]:    https://docs.microsoft.com/en-us/windows/console/writeconsoleinput
pub fn write_console_input(console_input: &mut impl AsConsoleInputHandle, buffer: &[INPUT_RECORD]) -> io::Result<usize> {
    let length = buffer.len().try_into().unwrap_or(!0);
    let mut written = 0;
    succeeded_to_result(unsafe { WriteConsoleInputW(console_input.as_raw_handle().cast(), buffer.as_ptr(), length, &mut written) })?;
    Ok(written as _ )
}

/// \[[WriteConsoleOutputW]\] Writes character and color attribute data to a specified rectangular block of character cells in a console screen buffer. The data to be written is taken from a correspondingly sized rectangular block at a specified location in the source buffer.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// use winapi::um::{wincontypes::*, winuser::*};
///
/// let mut output = CHAR_INFO::default();
/// unsafe { *output.Char.UnicodeChar_mut() = ' ' as u32 as u16 };
/// output.Attributes = FOREGROUND_GREEN.into();

/// let mut region = SMALL_RECT { Left: 10, Right: 11, Top: 20, Bottom: 21 };
///
/// write_console_output(&mut stdout(), &[output], (1,1), (0, 0), &mut region)?;
/// # Ok(())
/// # })();
/// ```
///
/// [WriteConsoleOutputW]:  https://docs.microsoft.com/en-us/windows/console/writeconsoleoutput
pub fn write_console_output(console_output: &mut impl AsConsoleOutputHandle, buffer: &[CHAR_INFO], buffer_size: impl IntoCoord, buffer_coord: impl IntoCoord, write_region: &mut SMALL_RECT) -> io::Result<()> {
    let console_output = console_output.as_raw_handle().cast();
    let buffer_size = buffer_size.into_coord();
    let buffer_coord = buffer_coord.into_coord();

    // Bounds checking
    if buffer_coord.X > buffer_size.X { return Err(io::Error::new(io::ErrorKind::InvalidInput, "write_console_output(): buffer_coord.X > buffer_size.X")); }
    if buffer_coord.Y > buffer_size.Y { return Err(io::Error::new(io::ErrorKind::InvalidInput, "write_console_output(): buffer_coord.Y > buffer_size.Y")); }
    let buffer_w = usize::try_from(buffer_size.X).map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "write_console_output(): buffer_size.X is out of SHORT bounds"))?;
    let buffer_h = usize::try_from(buffer_size.Y).map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "write_console_output(): buffer_size.Y is out of SHORT bounds"))?;
    let buffer_size_total = buffer_w.checked_mul(buffer_h).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "write_console_output(): buffer_size.X * Y is out of usize bounds"))?;
    if buffer_size_total > buffer.len() { return Err(io::Error::new(io::ErrorKind::InvalidInput, "write_console_output(): buffer_size is larger than buffer")); }
    // TODO: write_region bounds checking?

    succeeded_to_result(unsafe { WriteConsoleOutputW(console_output, buffer.as_ptr(), buffer_size, buffer_coord, write_region) })
}

/// \[[WriteConsoleOutputAttribute]\] Copies a number of character attributes to consecutive cells of a console screen buffer, beginning at a specified location.
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
/// [WriteConsoleOutputAttribute]:  https://docs.microsoft.com/en-us/windows/console/writeconsoleoutputattribute
pub fn write_console_output_attribute(console_output: &mut impl AsConsoleOutputHandle, attributes: &[Attributes], write_coord: impl IntoCoord) -> io::Result<usize> {
    let length = attributes.len().try_into().unwrap_or(!0);
    let mut written = 0;
    succeeded_to_result(unsafe { WriteConsoleOutputAttribute(console_output.as_raw_handle().cast(), attributes.as_ptr().cast(), length, write_coord.into_coord(), &mut written) })?;
    Ok(written as _)
}

/// \[[WriteConsoleOutputCharacterW]\] Copies a number of characters to consecutive cells of a console screen buffer, beginning at a specified location.
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
/// [WriteConsoleOutputCharacterW]: https://docs.microsoft.com/en-us/windows/console/writeconsoleoutputcharacter
pub fn write_console_output_character(console_output: &mut impl AsConsoleOutputHandle, characters: &[u16], write_coord: impl IntoCoord) -> io::Result<usize> {
    let length = characters.len().try_into().unwrap_or(!0);
    let mut written = 0;
    succeeded_to_result(unsafe { WriteConsoleOutputCharacterW(console_output.as_raw_handle().cast(), characters.as_ptr(), length, write_coord.into_coord(), &mut written) })?;
    Ok(written as _)
}
