use crate::*;

use winapi::shared::minwindef::UINT;
use winapi::um::consoleapi::*;
use winapi::um::wincon::*;

use std::fmt::{self, Debug, Formatter};
use std::io;



/// [Code Page Identifiers](https://docs.microsoft.com/en-us/windows/win32/intl/code-page-identifiers)
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CodePage(UINT);

impl CodePage {
    /// OEM United States
    pub const IBM437 : CodePage = CodePage(437);

    /// UTF7
    pub const UTF7 : CodePage = CodePage(65000);

    /// UTF8
    pub const UTF8 : CodePage = CodePage(65001);
}

impl Debug for CodePage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            CodePage::IBM437    => write!(f, "CodePage::IBM437 (OEM United States)"),
            CodePage::UTF7      => write!(f, "CodePage::UTF7"),
            CodePage::UTF8      => write!(f, "CodePage::UTF8"),
            CodePage(other)     => write!(f, "CodePage({})", other),
        }
    }
}

impl From<UINT> for CodePage { fn from(value: UINT) -> Self { Self(value) } }
impl From<CodePage> for UINT { fn from(value: CodePage) -> Self { value.0 } }




/// \[[GetConsoleCP]\] Retrieves the input code page used by the console associated with the calling process.
///
/// A console uses its input code page to translate keyboard input into the corresponding character value.
///
/// ### Example
/// ```no_run
/// # use maulingmonkey_console_winapi_wrappers::*;
/// assert_eq!(get_console_input_cp().unwrap(), CodePage::IBM437);
/// ```
///
/// [GetConsoleCP]:                     https://docs.microsoft.com/en-us/windows/console/getconsolecp
pub fn get_console_input_cp() -> io::Result<CodePage> {
    match unsafe { GetConsoleCP() } {
        0 => Err(io::Error::last_os_error()),
        n => Ok(CodePage(n)),
    }
}

/// \[[GetConsoleOutputCP]\] Retrieves the output code page used by the console associated with the calling process.
///
/// A console uses its output code page to translate the character values written by the various output functions into the images displayed in the console window.
///
/// ### Example
/// ```no_run
/// # use maulingmonkey_console_winapi_wrappers::*;
/// assert_eq!(get_console_output_cp().unwrap(), CodePage::IBM437);
/// ```
///
/// [GetConsoleOutputCP]:               https://docs.microsoft.com/en-us/windows/console/getconsoleoutputcp
pub fn get_console_output_cp() -> io::Result<CodePage> {
    match unsafe { GetConsoleOutputCP() } {
        0 => Err(io::Error::last_os_error()),
        n => Ok(CodePage(n)),
    }
}

/// \[[SetConsoleCP]\] Sets the input code page used by the console associated with the calling process.
///
/// A console uses its input code page to translate keyboard input into the corresponding character value.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # let _scope = InputCodePageScope::new(None);
/// set_console_input_cp(437).unwrap();
/// set_console_input_cp(CodePage::IBM437).unwrap();
/// set_console_input_cp(CodePage::from(437)).unwrap();
/// ```
///
/// [SetConsoleCP]:                     https://docs.microsoft.com/en-us/windows/console/setconsolecp
pub fn set_console_input_cp(codepage: impl Into<CodePage>) -> io::Result<()> {
    succeeded_to_result(unsafe{SetConsoleCP(codepage.into().0)})
}

/// \[[SetConsoleOutputCP]\] Sets the output code page used by the console associated with the calling process.
///
/// A console uses its output code page to translate the character values written by the various output functions into the images displayed in the console window.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # let _scope = OutputCodePageScope::new(None);
/// set_console_output_cp(437).unwrap();
/// set_console_output_cp(CodePage::IBM437).unwrap();
/// set_console_output_cp(CodePage::from(437)).unwrap();
/// ```
///
/// [SetConsoleOutputCP]:               https://docs.microsoft.com/en-us/windows/console/setconsoleoutputcp
pub fn set_console_output_cp(codepage: impl Into<CodePage>) -> io::Result<()> {
    succeeded_to_result(unsafe{SetConsoleOutputCP(codepage.into().0)})
}



/// [get_console_input_cp]\() [set_console_input_cp]\(new), [set_console_input_cp]\(old)
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// let _scope = InputCodePageScope::new(None);
/// let _scope = InputCodePageScope::new(CodePage::from(437));
/// let _scope = InputCodePageScope::new(CodePage::IBM437);
/// let _scope = InputCodePageScope::new(Some(CodePage::IBM437));
/// ```
#[derive(Debug)] pub struct InputCodePageScope  { old: CodePage }

/// [get_console_output_cp]\() [set_console_output_cp]\(new), [set_console_output_cp]\(old)
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// let _scope = OutputCodePageScope::new(None);
/// let _scope = OutputCodePageScope::new(CodePage::from(437));
/// let _scope = OutputCodePageScope::new(CodePage::IBM437);
/// let _scope = OutputCodePageScope::new(Some(CodePage::IBM437));
/// ```
#[derive(Debug)] pub struct OutputCodePageScope { old: CodePage }

impl InputCodePageScope {
    pub fn new(new: impl Into<Option<CodePage>>) -> io::Result<Self> {
        let scope = Self { old: get_console_input_cp()? };
        if let Some(new) = new.into() {
            set_console_input_cp(new)?;
        }
        Ok(scope)
    }
}

impl OutputCodePageScope {
    pub fn new(new: impl Into<Option<CodePage>>) -> io::Result<Self> {
        let scope = Self { old: get_console_output_cp()? };
        if let Some(new) = new.into() {
            set_console_output_cp(new)?;
        }
        Ok(scope)
    }
}

impl Drop for InputCodePageScope {
    fn drop(&mut self) {
        let result = set_console_input_cp(self.old);
        if cfg!(debug_assertions) {
            result.expect("Unable to restore input code page after InputCodePageScope");
        }
    }
}

impl Drop for OutputCodePageScope {
    fn drop(&mut self) {
        let result = set_console_output_cp(self.old);
        if cfg!(debug_assertions) {
            result.expect("Unable to restore output code page after OutputCodePageScope");
        }
    }
}
