use crate::*;

use winapi::shared::winerror::*;
use winapi::um::wincon::*;

use std::convert::*;
use std::ffi::*;
use std::fmt::{self, Debug, Formatter};
use std::io;
use std::ops::*;
use std::os::windows::prelude::*;
use std::ptr::*;



/// \[[AddConsoleAliasW]\] Defines a console alias for the specified executable.
///
/// ### Example
/// ```no_run
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # let _ = (|| -> std::io::Result<()> {
/// // https://docs.microsoft.com/en-us/windows/console/console-aliases
/// add_console_alias("test", r"cd \<a_very_long_path>\test", "cmd.exe")?;
/// # Ok(())
/// # })();
/// ```
///
/// [AddConsoleAliasW]: https://docs.microsoft.com/en-us/windows/console/addconsolealias
pub fn add_console_alias(source: impl AsRef<OsStr>, target: impl AsRef<OsStr>, exe_name: impl AsRef<OsStr>) -> io::Result<()> {
    // none of these are modified, AddConsoleAliasW just has bad const qualifications
    let mut source      = source    .as_ref().encode_wide().chain(Some(0)).collect::<Vec<_>>();
    let mut target      = target    .as_ref().encode_wide().chain(Some(0)).collect::<Vec<_>>();
    let mut exe_name    = exe_name  .as_ref().encode_wide().chain(Some(0)).collect::<Vec<_>>();
    succeeded_to_result(unsafe { AddConsoleAliasW(source.as_mut_ptr(), target.as_mut_ptr(), exe_name.as_mut_ptr()) })
}

/// \[[AddConsoleAliasW]\] Defines a console alias for the specified executable.
///
/// ### Example
/// ```no_run
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # let _ = (|| -> std::io::Result<()> {
/// // https://docs.microsoft.com/en-us/windows/console/console-aliases
/// clear_console_alias("test", (), "cmd.exe")?;
/// # Ok(())
/// # })();
/// ```
///
/// [AddConsoleAliasW]: https://docs.microsoft.com/en-us/windows/console/addconsolealias
pub fn clear_console_alias(source: impl AsRef<OsStr>, _target: (), exe_name: impl AsRef<OsStr>) -> io::Result<()> {
    // none of these are modified, AddConsoleAliasW just has bad const qualifications
    let mut source      = source    .as_ref().encode_wide().chain(Some(0)).collect::<Vec<_>>();
    let mut exe_name    = exe_name  .as_ref().encode_wide().chain(Some(0)).collect::<Vec<_>>();
    succeeded_to_result(unsafe { AddConsoleAliasW(source.as_mut_ptr(), null_mut(), exe_name.as_mut_ptr()) })
}



/// \[[GetConsoleAliasW]\] Retrieves the text for the specified console alias and executable.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::ffi::*;
/// # use std::os::windows::prelude::*;
/// # let _ = (|| -> std::io::Result<()> {
/// let alias = OsString::from(get_console_alias("test", &mut [0u16; 512], "cmd.exe")?);
/// # Ok(())
/// # })();
/// ```
///
/// [GetConsoleAliasW]: https://docs.microsoft.com/en-us/windows/console/getconsolealias
pub fn get_console_alias<'t>(source: impl AsRef<OsStr>, target_buffer: &'t mut impl AsMut<[u16]>, exe_name: impl AsRef<OsStr>) -> io::Result<TextRef<'t>> {
    let target_buffer   = target_buffer .as_mut();
    // none of these are modified, GetConsoleAliasW just has bad const qualifications
    let mut source      = source        .as_ref().encode_wide().chain(Some(0)).collect::<Vec<_>>();
    let mut exe_name    = exe_name      .as_ref().encode_wide().chain(Some(0)).collect::<Vec<_>>();
    let bytes = unsafe { GetConsoleAliasW(source.as_mut_ptr(), target_buffer.as_mut_ptr(), target_buffer.len() as _, exe_name.as_mut_ptr()) };
    if bytes == 0 { last_os_error_unless_success()? }
    Ok(TextRef(&target_buffer[..(bytes/2) as _].strip_suffix(&[0]).ok_or_else(|| io::Error::from_raw_os_error(ERROR_NOT_ENOUGH_MEMORY as _))?))
}



/// \[[GetConsoleAliasesW]\] Retrieves all defined console aliases for the specified executable.
///
/// Separates keys and values with `=`.  As keys can also contain `=`s, this can result in ambiguous aliases!  Given:
///
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// let exe = "maulingmonkey-console-winapi-wrappers-test.exe";
/// dbg!(get_console_aliases(&mut [0u16; 512], exe).unwrap().collect::<Vec<_>>());
/// ```
///
/// ```text
/// [src\alias.rs:???] get_console_aliases(&mut [0u16; 512], exe).unwrap().collect::<Vec<_>>() = [
///     TextRef("test-alias1=alias1target"),
///     TextRef("test-alias2=alias2target"),
///     TextRef("test=equal=value=value"),
/// ]
/// ```
///
/// The last entry could've been added by any of:
///
/// | Key                   | Value |
/// | --------------------- | ----- |
/// | `"test"`              | `"equal=value=value"`
/// | `"test=equal"`        | `"value=value"`
/// | `"test=equal=value"`  | `"value"`
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # let _ = (|| -> std::io::Result<()> {
/// let exe = "cmd.exe";
/// let mut aliases = vec![0u16; get_console_aliases_length(exe).wchars()];
/// for alias in get_console_aliases(&mut aliases, exe)? {
///     dbg!(alias.to_os_string());
/// }
/// # Ok(())
/// # })();
/// ```
///
/// [GetConsoleAliasesW]: https://docs.microsoft.com/en-us/windows/console/getconsolealiases
pub fn get_console_aliases<'t>(alias_buffer: &'t mut impl AsMut<[u16]>, exe_name: impl AsRef<OsStr>) -> io::Result<TextNsvRef<'t>> {
    let alias_buffer    = alias_buffer.as_mut();
    // none of these are modified, GetConsoleAliasW just has bad const qualifications
    let mut exe_name    = exe_name  .as_ref().encode_wide().chain(Some(0)).collect::<Vec<_>>();
    let bytes = unsafe { GetConsoleAliasesW(alias_buffer.as_mut_ptr(), alias_buffer.len() as _, exe_name.as_mut_ptr()) };
    if bytes == 0 { last_os_error_unless_success()? }
    Ok(TextNsvRef(&alias_buffer[..(bytes/2) as _]))
}

// TODO: get_console_aliases_vec? get_console_aliases_inplace?

/// \[[GetConsoleAliasesLengthW]\] Retrieves the required size for the buffer, **in bytes**, for use by the [get_console_aliases] function.
///
/// [GetConsoleAliasesLengthW]: https://docs.microsoft.com/en-us/windows/console/getconsolealiaseslength
pub fn get_console_aliases_length(exe_name: impl AsRef<OsStr>) -> LengthBytesOrWchars {
    // none of these are modified, GetConsoleAliasesLengthW just has bad const qualifications
    let mut exe_name = exe_name.as_ref().encode_wide().chain(Some(0)).collect::<Vec<_>>();
    LengthBytesOrWchars(unsafe { GetConsoleAliasesLengthW(exe_name.as_mut_ptr()) as _ })
}



/// \[[GetConsoleAliasExesW]\] Retrieves the names of all executable files with console aliases defined.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # let _ = (|| -> std::io::Result<()> {
/// let mut exes = vec![0u16; get_console_alias_exes_length().wchars()];
/// for exe in get_console_alias_exes(&mut exes)? {
///     dbg!(exe.to_os_string());
/// }
/// # Ok(())
/// # })();
/// ```
///
/// [GetConsoleAliasExesW]: https://docs.microsoft.com/en-us/windows/console/getconsolealiasexes
pub fn get_console_alias_exes(exe_name_buffer: &mut impl AsMut<[u16]>) -> io::Result<TextNsvRef> {
    let exe_name_buffer = exe_name_buffer.as_mut();
    let bytes = unsafe { GetConsoleAliasExesW(exe_name_buffer.as_mut_ptr(), exe_name_buffer.len() as _) };
    if bytes == 0 { last_os_error_unless_success()? }
    Ok(TextNsvRef(&exe_name_buffer[..(bytes/2) as _]))
}

/// \[[GetConsoleAliasExesLengthW]\] Retrieves the required size for the buffer used by the [get_console_alias_exes] function.
///
/// [GetConsoleAliasExesLengthW]: https://docs.microsoft.com/en-us/windows/console/getconsolealiasexeslength
pub fn get_console_alias_exes_length() -> LengthBytesOrWchars {
    LengthBytesOrWchars(unsafe { GetConsoleAliasExesLengthW() as _ })
}



#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)] pub struct TextRef<'a>(&'a [u16]);

impl<'a> TextRef<'a> {
    pub fn as_wchars(self) -> &'a [u16] { self.0 }
    pub fn to_os_string(self) -> OsString { OsString::from_wide(self.0) }
    pub fn to_string(self) -> Result<String, OsString> { OsString::from_wide(self.0).into_string() }
}

impl Debug for TextRef<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { write!(f, "TextRef({:?})", OsString::from(*self)) }
}

impl<'a> AsRef<[u16]> for TextRef<'a> {
    fn as_ref(&self) -> &[u16] { self.0 }
}

impl<'a> Deref for TextRef<'a> {
    type Target = [u16];
    fn deref(&self) -> &Self::Target { self.0 }
}

impl From<TextRef<'_>> for OsString {
    fn from(r: TextRef<'_>) -> Self { OsString::from_wide(r.0) }
}

impl TryFrom<TextRef<'_>> for String {
    type Error = OsString;
    fn try_from(r: TextRef<'_>) -> Result<Self, Self::Error> { OsString::from_wide(r.0).into_string() }
}



#[derive(Clone, Copy, Debug)] pub struct TextNsvRef<'a>(&'a [u16]);

impl<'a> TextNsvRef<'a> {
}

impl<'a> AsRef<[u16]> for TextNsvRef<'a> {
    fn as_ref(&self) -> &[u16] { self.0 }
}

impl<'a> Deref for TextNsvRef<'a> {
    type Target = [u16];
    fn deref(&self) -> &Self::Target { self.0 }
}

impl<'a> Iterator for TextNsvRef<'a> {
    type Item = TextRef<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let (result, rest) = if self.is_empty() {
            (None, &[][..])
        } else if let Some(nul) = self.0.iter().copied().position(|ch| ch == 0u16) {
            (Some(&self.0[..nul]), &self.0[nul+1..])
        } else {
            (Some(self.0), &[][..])
        };
        self.0 = rest;
        result.map(TextRef)
    }
}



#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)] pub struct LengthBytesOrWchars(usize);

impl LengthBytesOrWchars {
    pub fn bytes(&self) -> usize { self.0 }
    pub fn wchars(&self) -> usize { (self.0/2) + (self.0&1) }
}



#[test] fn aliases() {
    use wchar::wch;

    let exe = "maulingmonkey-console-winapi-wrappers-test.exe";
    let bad_exe = "maulingmonkey-console-winapi-wrappers-bad.exe"; // never set

    clear_console_alias("test-alias1", (), exe).unwrap();
    clear_console_alias("test-alias1", (), exe).unwrap(); // no error despite removing a removed alias
    clear_console_alias("test-never",  (), exe).unwrap(); // no error despite removing a never-existing alias
    clear_console_alias("test-alias2", (), exe).unwrap();
    clear_console_alias("test=equal", (), exe).unwrap(); // keys containins `=`s are super gross

    add_console_alias("test-alias1", "old", exe).unwrap();
    add_console_alias("test-alias1", "alias1target", exe).unwrap(); // no error despite overwriting an alias
    add_console_alias("test-alias2", "alias2target", exe).unwrap();
    add_console_alias("test=equal", "value=value", exe).unwrap(); // keys containins `=`s are super gross

    add_console_alias("test-removed", "temp", exe).unwrap();
    clear_console_alias("test-removed", (), exe).unwrap();



    let err = get_console_alias("test-never",  &mut [0u16; 512], exe).unwrap_err();
    assert_eq!(err.raw_os_error(), Some(31));
    assert_eq!(err.raw_os_error(), Some(ERROR_GEN_FAILURE as _));
    assert_eq!(err.kind(), io::ErrorKind::Other);

    let err = get_console_alias("test-removed",  &mut [0u16; 512], exe).unwrap_err();
    assert_eq!(err.raw_os_error(), Some(31));
    assert_eq!(err.raw_os_error(), Some(ERROR_GEN_FAILURE as _));
    assert_eq!(err.kind(), io::ErrorKind::Other);

    let alias1 = get_console_alias("test-alias1", &mut [0u16; 512], exe).unwrap().to_os_string();
    assert_eq!(alias1, "alias1target");

    let alias2 = get_console_alias("test-alias2", &mut [0u16; 512], exe).unwrap().to_os_string();
    assert_eq!(alias2, "alias2target");

    let equal = get_console_alias("test=equal", &mut [0u16; 512], exe).unwrap().to_os_string();
    assert_eq!(equal, "value=value");

    get_console_alias("test", &mut [0u16; 512], exe).unwrap_err();
    get_console_alias("test=equal", &mut [0u16; 512], exe).unwrap();
    get_console_alias("test=equal=value", &mut [0u16; 512], exe).unwrap_err();



    let mut aliases = [0u16; 512];
    let mut aliases = get_console_aliases(&mut aliases, exe).unwrap().collect::<Vec<_>>();
    aliases.sort();
    assert_eq!(aliases.len(), 3);
    assert!(aliases[0].as_wchars() == wch!("test-alias1=alias1target"));
    assert!(aliases[1].as_wchars() == wch!("test-alias2=alias2target"));
    assert!(aliases[2].as_wchars() == wch!("test=equal=value=value"));

    assert_eq!(get_console_aliases_length(exe).wchars(), aliases.iter().map(|a| a.len()+1).sum());

    let mut exes = [0u16; 512];
    let exes = get_console_alias_exes(&mut exes).unwrap().collect::<Vec<_>>();
    assert!(exes.contains(&TextRef(wch!("maulingmonkey-console-winapi-wrappers-test.exe"))));

    clear_console_alias("test-alias1", (), exe).unwrap();
    clear_console_alias("test-alias2", (), exe).unwrap();
    clear_console_alias("test=equal",  (), exe).unwrap();

    // error if all aliases removed
    let err = get_console_aliases(&mut [0u16; 512], exe).unwrap_err();
    assert_eq!(err.raw_os_error(), Some(31));
    assert_eq!(err.raw_os_error(), Some(ERROR_GEN_FAILURE as _));
    assert_eq!(err.kind(), io::ErrorKind::Other);

    // error if no aliases ever present
    let err = get_console_aliases(&mut [0u16; 512], bad_exe).unwrap_err();
    assert_eq!(err.raw_os_error(), Some(31));
    assert_eq!(err.raw_os_error(), Some(ERROR_GEN_FAILURE as _));
    assert_eq!(err.kind(), io::ErrorKind::Other);
}

fn last_os_error_unless_success() -> io::Result<()> {
    let err = io::Error::last_os_error();
    match err.raw_os_error() {
        Some(0) => Ok(()),
        _ => Err(err),
    }
}