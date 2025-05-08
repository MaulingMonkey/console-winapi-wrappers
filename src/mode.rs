use crate::*;

use winapi::um::consoleapi::*;
use winapi::um::wincon;

use std::ops::*;
use std::io;



#[doc(alias = "GetConsoleMode")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/getconsolemode)\]
/// Retrieves the current mode of a console's input or screen buffer.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// let input_mode : InputMode = get_console_mode(&std::io::stdin())?;
/// let output_mode : OutputMode = get_console_mode(&std::io::stdout())?;
/// # Ok(())
/// # })();
/// ```
///
/// ### See Also
/// * [InputMode]
/// * [OutputMode]
///
pub fn get_console_mode<CH: AsConsoleHandle>(handle: &CH) -> io::Result<CH::Mode> {
    let mut mode = 0;
    succeeded_to_result(unsafe { GetConsoleMode(handle.as_raw_handle().cast(), &mut mode) })?;
    Ok(mode.into())
}

#[doc(alias = "SetConsoleMode")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/setconsolemode)\]
/// Sets the mode of a console's input or screen buffer.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// # let input_mode : InputMode = get_console_mode(&std::io::stdin())?;
/// # let output_mode : OutputMode = get_console_mode(&std::io::stdout())?;
/// set_console_mode(&mut std::io::stdin(), input_mode | ENABLE_MOUSE_INPUT)?;
/// set_console_mode(&mut std::io::stdout(), output_mode | ENABLE_PROCESSED_OUTPUT)?;
/// # Ok(())
/// # })();
/// ```
///
/// ### See Also
/// * [InputMode]
/// * [OutputMode]
///
pub fn set_console_mode<CH: AsConsoleHandle>(handle: &mut CH, mode: CH::Mode) -> io::Result<()> {
    succeeded_to_result(unsafe { SetConsoleMode(handle.as_raw_handle().cast(), mode.into()) })
}

#[doc(alias = "GetConsoleMode")]
#[doc(alias = "SetConsoleMode")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/setconsolemode)\]
/// Retrieves and changes the current mode of a console's input or screen buffer.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::io::{self, *};
/// # let _ = (|| -> io::Result<()> {
/// change_console_mode(&mut std::io::stdin(), |mode| mode | ENABLE_MOUSE_INPUT)?;
/// change_console_mode(&mut std::io::stdout(), |mode| mode | ENABLE_PROCESSED_OUTPUT)?;
/// # Ok(())
/// # })();
/// ```
///
/// ### See Also
/// * [InputMode]
/// * [OutputMode]
///
pub fn change_console_mode<CH: AsConsoleHandle>(handle: &mut CH, map: impl FnOnce(CH::Mode) -> CH::Mode) -> io::Result<()> {
    let mode = get_console_mode(handle)?;
    let mode = map(mode);
    set_console_mode(handle, mode)
}



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/getconsolemode)\]
/// Console modes applicable to input handles
///
#[repr(transparent)] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)] pub struct InputMode(u32);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/getconsolemode)\]
/// Console modes applicable to output handles / screen buffers
///
#[repr(transparent)] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)] pub struct OutputMode(u32);

impl From<u32> for InputMode  { fn from(value: u32) -> Self { Self(value) } }
impl From<u32> for OutputMode { fn from(value: u32) -> Self { Self(value) } }
impl From<InputMode > for u32 { fn from(value: InputMode ) -> Self { value.0 } }
impl From<OutputMode> for u32 { fn from(value: OutputMode) -> Self { value.0 } }

impl BitAnd for InputMode  { type Output = Self; fn bitand(self, rhs: Self) -> Self::Output { Self(self.0 & rhs.0) } }
impl BitAnd for OutputMode { type Output = Self; fn bitand(self, rhs: Self) -> Self::Output { Self(self.0 & rhs.0) } }
impl BitXor for InputMode  { type Output = Self; fn bitxor(self, rhs: Self) -> Self::Output { Self(self.0 ^ rhs.0) } }
impl BitXor for OutputMode { type Output = Self; fn bitxor(self, rhs: Self) -> Self::Output { Self(self.0 ^ rhs.0) } }
impl BitOr  for InputMode  { type Output = Self; fn bitor (self, rhs: Self) -> Self::Output { Self(self.0 | rhs.0) } }
impl BitOr  for OutputMode { type Output = Self; fn bitor (self, rhs: Self) -> Self::Output { Self(self.0 | rhs.0) } }
impl Not    for InputMode  { type Output = Self; fn not   (self)            -> Self::Output { Self(!self.0) } }
impl Not    for OutputMode { type Output = Self; fn not   (self)            -> Self::Output { Self(!self.0) } }

impl BitAndAssign for InputMode  { fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; } }
impl BitAndAssign for OutputMode { fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; } }
impl BitXorAssign for InputMode  { fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; } }
impl BitXorAssign for OutputMode { fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; } }
impl BitOrAssign  for InputMode  { fn bitor_assign (&mut self, rhs: Self) { self.0 |= rhs.0; } }
impl BitOrAssign  for OutputMode { fn bitor_assign (&mut self, rhs: Self) { self.0 |= rhs.0; } }



/// Characters read by the [ReadFile] or [ReadConsole] function are written to the active screen buffer as they
/// are typed into the console. This mode can be used only if the [`ENABLE_LINE_INPUT`] mode is also
/// enabled.
///
/// [ReadFile]:     https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile
/// [ReadConsole]:  https://learn.microsoft.com/en-us/windows/console/readconsole
#[doc(hidden)]
pub const ENABLE_ECHO_INPUT : InputMode = InputMode(wincon::ENABLE_ECHO_INPUT);

/// When enabled, text entered in a console window will be inserted at the current cursor location and all
/// text following that location will not be overwritten. When disabled, all following text will be overwritten.
#[doc(hidden)]
pub const ENABLE_INSERT_MODE : InputMode = InputMode(wincon::ENABLE_INSERT_MODE);

/// The [ReadFile] or [ReadConsole] function returns only when a carriage return character is read. If this
/// mode is disabled, the functions return when one or more characters are available.
///
/// [ReadFile]:     https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile
/// [ReadConsole]:  https://learn.microsoft.com/en-us/windows/console/readconsole
#[doc(hidden)]
pub const ENABLE_LINE_INPUT : InputMode = InputMode(wincon::ENABLE_LINE_INPUT);

/// If the mouse pointer is within the borders of the console window and the window has the keyboard
/// focus, mouse events generated by mouse movement and button presses are placed in the input buffer.
/// These events are discarded by [ReadFile] or [ReadConsole], even when this mode is enabled.
///
/// [ReadFile]:     https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile
/// [ReadConsole]:  https://learn.microsoft.com/en-us/windows/console/readconsole
#[doc(hidden)]
pub const ENABLE_MOUSE_INPUT : InputMode = InputMode(wincon::ENABLE_MOUSE_INPUT);

/// CTRL+C is processed by the system and is not placed in the input buffer. If the input buffer is being
/// read by [ReadFile] or [ReadConsole], other control keys are processed by the system and are not returned
/// in the [ReadFile] or [ReadConsole] buffer. If the [`ENABLE_LINE_INPUT`] mode is also enabled, backspace,
/// carriage return, and line feed characters are handled by the system.
///
/// [ReadFile]:     https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile
/// [ReadConsole]:  https://learn.microsoft.com/en-us/windows/console/readconsole
#[doc(hidden)]
pub const ENABLE_PROCESSED_INPUT : InputMode = InputMode(wincon::ENABLE_PROCESSED_INPUT);

/// This flag enables the user to use the mouse to select and edit text. To enable this mode, use
/// <code>[ENABLE_QUICK_EDIT_MODE] | [ENABLE_EXTENDED_FLAGS]</code>. To disable this mode, use
/// [`ENABLE_EXTENDED_FLAGS`] without this flag.
#[doc(hidden)]
pub const ENABLE_QUICK_EDIT_MODE : InputMode = InputMode(wincon::ENABLE_QUICK_EDIT_MODE);

/// ???
#[doc(hidden)]
pub const ENABLE_EXTENDED_FLAGS : InputMode = InputMode(wincon::ENABLE_EXTENDED_FLAGS);

/// User interactions that change the size of the console screen buffer are reported in the console's input
/// buffer. Information about these events can be read from the input buffer by applications using the
/// [ReadConsoleInput] function, but not by those using [ReadFile] or [ReadConsole].
///
/// [ReadConsoleInput]: https://learn.microsoft.com/en-us/windows/console/readconsoleinput
/// [ReadFile]:         https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile
/// [ReadConsole]:      https://learn.microsoft.com/en-us/windows/console/readconsole
#[doc(hidden)]
pub const ENABLE_WINDOW_INPUT : InputMode = InputMode(wincon::ENABLE_WINDOW_INPUT);

/// Setting this flag directs the Virtual Terminal processing engine to convert user input received by the
/// console window into [Console Virtual Terminal Sequences] that can be retrieved by a supporting
/// application through [WriteFile] or [WriteConsole] functions.
///
/// The typical usage of this flag is intended in conjunction with
/// [`ENABLE_VIRTUAL_TERMINAL_PROCESSING`] on the output handle to connect to an application that
/// communicates exclusively via virtual terminal sequences.
///
/// [Console Virtual Terminal Sequences]:   https://learn.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences
/// [WriteFile]:                            https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-writefile
/// [WriteConsole]:                         https://learn.microsoft.com/en-us/windows/console/writeconsole
#[doc(hidden)]
pub const ENABLE_VIRTUAL_TERMINAL_INPUT : InputMode = InputMode(wincon::ENABLE_VIRTUAL_TERMINAL_INPUT);

impl InputMode {
    /// Characters read by the [ReadFile] or [ReadConsole] function are written to the active screen buffer as they
    /// are typed into the console. This mode can be used only if the ENABLE_LINE_INPUT mode is also
    /// enabled.
    ///
    /// [ReadFile]:     https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile
    /// [ReadConsole]:  https://learn.microsoft.com/en-us/windows/console/readconsole
    pub const ENABLE_ECHO_INPUT : InputMode = ENABLE_ECHO_INPUT;

    /// When enabled, text entered in a console window will be inserted at the current cursor location and all
    /// text following that location will not be overwritten. When disabled, all following text will be overwritten.
    pub const ENABLE_INSERT_MODE : InputMode = ENABLE_INSERT_MODE;

    /// The [ReadFile] or [ReadConsole] function returns only when a carriage return character is read. If this
    /// mode is disabled, the functions return when one or more characters are available.
    ///
    /// [ReadFile]:     https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile
    /// [ReadConsole]:  https://learn.microsoft.com/en-us/windows/console/readconsole
    pub const ENABLE_LINE_INPUT : InputMode = ENABLE_LINE_INPUT;

    /// If the mouse pointer is within the borders of the console window and the window has the keyboard
    /// focus, mouse events generated by mouse movement and button presses are placed in the input buffer.
    /// These events are discarded by [ReadFile] or [ReadConsole], even when this mode is enabled.
    ///
    /// [ReadFile]:     https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile
    /// [ReadConsole]:  https://learn.microsoft.com/en-us/windows/console/readconsole
    pub const ENABLE_MOUSE_INPUT : InputMode = ENABLE_MOUSE_INPUT;

    /// CTRL+C is processed by the system and is not placed in the input buffer. If the input buffer is being
    /// read by [ReadFile] or [ReadConsole], other control keys are processed by the system and are not returned
    /// in the [ReadFile] or [ReadConsole] buffer. If the [ENABLE_LINE_INPUT] mode is also enabled, backspace,
    /// carriage return, and line feed characters are handled by the system.
    ///
    /// [ReadFile]:     https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile
    /// [ReadConsole]:  https://learn.microsoft.com/en-us/windows/console/readconsole
    pub const ENABLE_PROCESSED_INPUT : InputMode = ENABLE_PROCESSED_INPUT;

    /// This flag enables the user to use the mouse to select and edit text. To enable this mode, use
    /// <code>[ENABLE_QUICK_EDIT_MODE] | [ENABLE_EXTENDED_FLAGS]</code>. To disable this mode, use
    /// [`ENABLE_EXTENDED_FLAGS`] without this flag.
    pub const ENABLE_QUICK_EDIT_MODE : InputMode = ENABLE_QUICK_EDIT_MODE;

    /// ???
    pub const ENABLE_EXTENDED_FLAGS : InputMode = ENABLE_EXTENDED_FLAGS;

    /// User interactions that change the size of the console screen buffer are reported in the console's input
    /// buffer. Information about these events can be read from the input buffer by applications using the
    /// [ReadConsoleInput] function, but not by those using [ReadFile] or [ReadConsole].
    ///
    /// [ReadConsoleInput]: https://learn.microsoft.com/en-us/windows/console/readconsoleinput
    /// [ReadFile]:         https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile
    /// [ReadConsole]:      https://learn.microsoft.com/en-us/windows/console/readconsole
    pub const ENABLE_WINDOW_INPUT : InputMode = ENABLE_WINDOW_INPUT;

    /// Setting this flag directs the Virtual Terminal processing engine to convert user input received by the
    /// console window into [Console Virtual Terminal Sequences] that can be retrieved by a supporting
    /// application through [WriteFile] or [WriteConsole] functions.
    ///
    /// The typical usage of this flag is intended in conjunction with
    /// [`ENABLE_VIRTUAL_TERMINAL_PROCESSING`] on the output handle to connect to an application that
    /// communicates exclusively via virtual terminal sequences.
    ///
    /// [Console Virtual Terminal Sequences]:   https://learn.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences
    /// [WriteFile]:                            https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-writefile
    /// [WriteConsole]:                         https://learn.microsoft.com/en-us/windows/console/writeconsole
    pub const ENABLE_VIRTUAL_TERMINAL_INPUT : InputMode = ENABLE_VIRTUAL_TERMINAL_INPUT;
}



/// Characters written by the [WriteFile] or [WriteConsole] function or echoed by the [ReadFile] or
/// [ReadConsole] function are parsed for ASCII control sequences, and the correct action is
/// performed. Backspace, tab, bell, carriage return, and line feed characters are processed.
///
/// [WriteFile]:    https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-writefile
/// [WriteConsole]: https://learn.microsoft.com/en-us/windows/console/writeconsole
/// [ReadFile]:     https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile
/// [ReadConsole]:  https://learn.microsoft.com/en-us/windows/console/readconsole
#[doc(hidden)]
pub const ENABLE_PROCESSED_OUTPUT : OutputMode = OutputMode(wincon::ENABLE_PROCESSED_OUTPUT);

/// When writing with [WriteFile] or [WriteConsole] or echoing with [ReadFile] or [ReadConsole], the
/// cursor moves to the beginning of the next row when it reaches the end of the current row. This
/// causes the rows displayed in the console window to scroll up automatically when the cursor
/// advances beyond the last row in the window. It also causes the contents of the console screen
/// buffer to scroll up (discarding the top row of the console screen buffer) when the cursor
/// advances beyond the last row in the console screen buffer. If this mode is disabled, the last
/// character in the row is overwritten with any subsequent characters.
///
/// [WriteFile]:    https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-writefile
/// [WriteConsole]: https://learn.microsoft.com/en-us/windows/console/writeconsole
/// [ReadFile]:     https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile
/// [ReadConsole]:  https://learn.microsoft.com/en-us/windows/console/readconsole
#[doc(hidden)]
pub const ENABLE_WRAP_AT_EOL_OUTPUT : OutputMode = OutputMode(wincon::ENABLE_WRAP_AT_EOL_OUTPUT);

/// When writing with [WriteFile] or [WriteConsole], characters are parsed for VT100 and similar
/// control character sequences that control cursor movement, color/font mode, and other
/// operations that can also be performed via the existing Console APIs. For more information, see
/// [Console Virtual Terminal Sequences].
///
/// [WriteFile]:                            https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-writefile
/// [WriteConsole]:                         https://learn.microsoft.com/en-us/windows/console/writeconsole
/// [Console Virtual Terminal Sequences]:   https://learn.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences
#[doc(hidden)]
pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING : OutputMode = OutputMode(wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING);

/// When writing with [WriteFile] or [WriteConsole], this adds an additional state to end-of-line
/// wrapping that can delay the cursor move and buffer scroll operations.
///
/// Normally when [`ENABLE_WRAP_AT_EOL_OUTPUT`] is set and text reaches the end of the line, the
/// cursor will immediately move to the next line and the contents of the buffer will scroll up by one
/// line. In contrast with this flag set, the scroll operation and cursor move is delayed until the next
/// character arrives. The written character will be printed in the final position on the line and the
/// cursor will remain above this character as if [`ENABLE_WRAP_AT_EOL_OUTPUT`] was off, but the
/// next printable character will be printed as if [`ENABLE_WRAP_AT_EOL_OUTPUT`] is on. No overwrite
/// will occur. Specifically, the cursor quickly advances down to the following line, a scroll is
/// performed if necessary, the character is printed, and the cursor advances one more position.
///
/// The typical usage of this flag is intended in conjunction with setting
/// [`ENABLE_VIRTUAL_TERMINAL_PROCESSING`] to better emulate a terminal emulator where writing
/// the final character on the screen (in the bottom right corner) without triggering an immediate
/// scroll is the desired behavior.
///
/// [WriteFile]:    https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-writefile
/// [WriteConsole]: https://learn.microsoft.com/en-us/windows/console/writeconsole
#[doc(hidden)]
pub const DISABLE_NEWLINE_AUTO_RETURN : OutputMode = OutputMode(wincon::DISABLE_NEWLINE_AUTO_RETURN);

/// The APIs for writing character attributes including [WriteConsoleOutput] and
/// [WriteConsoleOutputAttribute] allow the usage of flags from [character attributes] to adjust the
/// color of the foreground and background of text. Additionally, a range of DBCS flags was
/// specified with the COMMON_LVB prefix. Historically, these flags only functioned in DBCS code
/// pages for Chinese, Japanese, and Korean languages.
///
/// With exception of the leading byte and trailing byte flags, the remaining flags describing line
/// drawing and reverse video (../swap foreground and background colors) can be useful for other
/// languages to emphasize portions of output.
///
/// Setting this console mode flag will allow these attributes to be used in every code page on every
/// language.
///
/// It is off by default to maintain compatibility with known applications that have historically taken
/// advantage of the console ignoring these flags on non-CJK machines to store bits in these fields
/// for their own purposes or by accident.
///
/// Note that using the [`ENABLE_VIRTUAL_TERMINAL_PROCESSING`] mode can result in LVB grid and
/// reverse video flags being set while this flag is still off if the attached application requests
/// underlining or inverse video via [Console Virtual Terminal Sequences].
///
/// [WriteConsoleOutput]:                   https://learn.microsoft.com/en-us/windows/console/writeconsoleoutput
/// [WriteConsoleOutputAttribute]:          https://learn.microsoft.com/en-us/windows/console/writeconsoleoutputattribute
/// [character attributes]:                 https://learn.microsoft.com/en-us/windows/console/console-screen-buffers#character-attributes
/// [Console Virtual Terminal Sequences]:   https://learn.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences
#[doc(hidden)]
pub const ENABLE_LVB_GRID_WORLDWIDE : OutputMode = OutputMode(wincon::ENABLE_LVB_GRID_WORLDWIDE);

impl OutputMode {
    /// Characters written by the [WriteFile] or [WriteConsole] function or echoed by the [ReadFile] or
    /// [ReadConsole] function are parsed for ASCII control sequences, and the correct action is
    /// performed. Backspace, tab, bell, carriage return, and line feed characters are processed.
    ///
    /// [WriteFile]:    https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-writefile
    /// [WriteConsole]: https://learn.microsoft.com/en-us/windows/console/writeconsole
    /// [ReadFile]:     https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile
    /// [ReadConsole]:  https://learn.microsoft.com/en-us/windows/console/readconsole
    pub const ENABLE_PROCESSED_OUTPUT : OutputMode = ENABLE_PROCESSED_OUTPUT;

    /// When writing with [WriteFile] or [WriteConsole] or echoing with [ReadFile] or [ReadConsole], the
    /// cursor moves to the beginning of the next row when it reaches the end of the current row. This
    /// causes the rows displayed in the console window to scroll up automatically when the cursor
    /// advances beyond the last row in the window. It also causes the contents of the console screen
    /// buffer to scroll up (discarding the top row of the console screen buffer) when the cursor
    /// advances beyond the last row in the console screen buffer. If this mode is disabled, the last
    /// character in the row is overwritten with any subsequent characters.
    ///
    /// [WriteFile]:    https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-writefile
    /// [WriteConsole]: https://learn.microsoft.com/en-us/windows/console/writeconsole
    /// [ReadFile]:     https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile
    /// [ReadConsole]:  https://learn.microsoft.com/en-us/windows/console/readconsole
    pub const ENABLE_WRAP_AT_EOL_OUTPUT : OutputMode = ENABLE_WRAP_AT_EOL_OUTPUT;

    /// When writing with [WriteFile] or [WriteConsole], characters are parsed for VT100 and similar
    /// control character sequences that control cursor movement, color/font mode, and other
    /// operations that can also be performed via the existing Console APIs. For more information, see
    /// [Console Virtual Terminal Sequences].
    ///
    /// [WriteFile]:                            https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-writefile
    /// [WriteConsole]:                         https://learn.microsoft.com/en-us/windows/console/writeconsole
    /// [Console Virtual Terminal Sequences]:   https://learn.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences
    pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING : OutputMode = ENABLE_VIRTUAL_TERMINAL_PROCESSING;

    /// When writing with [WriteFile] or [WriteConsole], this adds an additional state to end-of-line
    /// wrapping that can delay the cursor move and buffer scroll operations.
    ///
    /// Normally when [`ENABLE_WRAP_AT_EOL_OUTPUT`] is set and text reaches the end of the line, the
    /// cursor will immediately move to the next line and the contents of the buffer will scroll up by one
    /// line. In contrast with this flag set, the scroll operation and cursor move is delayed until the next
    /// character arrives. The written character will be printed in the final position on the line and the
    /// cursor will remain above this character as if [`ENABLE_WRAP_AT_EOL_OUTPUT`] was off, but the
    /// next printable character will be printed as if [`ENABLE_WRAP_AT_EOL_OUTPUT`] is on. No overwrite
    /// will occur. Specifically, the cursor quickly advances down to the following line, a scroll is
    /// performed if necessary, the character is printed, and the cursor advances one more position.
    ///
    /// The typical usage of this flag is intended in conjunction with setting
    /// [`ENABLE_VIRTUAL_TERMINAL_PROCESSING`] to better emulate a terminal emulator where writing
    /// the final character on the screen (in the bottom right corner) without triggering an immediate
    /// scroll is the desired behavior.
    ///
    /// [WriteFile]:    https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-writefile
    /// [WriteConsole]: https://learn.microsoft.com/en-us/windows/console/writeconsole
    pub const DISABLE_NEWLINE_AUTO_RETURN : OutputMode = DISABLE_NEWLINE_AUTO_RETURN;

    /// The APIs for writing character attributes including [WriteConsoleOutput] and
    /// [WriteConsoleOutputAttribute] allow the usage of flags from [character attributes] to adjust the
    /// color of the foreground and background of text. Additionally, a range of DBCS flags was
    /// specified with the COMMON_LVB prefix. Historically, these flags only functioned in DBCS code
    /// pages for Chinese, Japanese, and Korean languages.
    ///
    /// With exception of the leading byte and trailing byte flags, the remaining flags describing line
    /// drawing and reverse video (../swap foreground and background colors) can be useful for other
    /// languages to emphasize portions of output.
    ///
    /// Setting this console mode flag will allow these attributes to be used in every code page on every
    /// language.
    ///
    /// It is off by default to maintain compatibility with known applications that have historically taken
    /// advantage of the console ignoring these flags on non-CJK machines to store bits in these fields
    /// for their own purposes or by accident.
    ///
    /// Note that using the [`ENABLE_VIRTUAL_TERMINAL_PROCESSING`] mode can result in LVB grid and
    /// reverse video flags being set while this flag is still off if the attached application requests
    /// underlining or inverse video via [Console Virtual Terminal Sequences].
    ///
    /// [WriteConsoleOutput]:                   https://learn.microsoft.com/en-us/windows/console/writeconsoleoutput
    /// [WriteConsoleOutputAttribute]:          https://learn.microsoft.com/en-us/windows/console/writeconsoleoutputattribute
    /// [character attributes]:                 https://learn.microsoft.com/en-us/windows/console/console-screen-buffers#character-attributes
    /// [Console Virtual Terminal Sequences]:   https://learn.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences
    pub const ENABLE_LVB_GRID_WORLDWIDE : OutputMode = ENABLE_LVB_GRID_WORLDWIDE;
}
