use crate::*;

use winapi::shared::winerror::*;
use winapi::um::errhandlingapi::*;
use winapi::um::wincon::*;

use std::convert::*;
use std::ffi::*;
use std::fmt::{self, Debug, Formatter};
use std::io;
use std::mem::size_of_val;
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
    let mut source      = widen0(source     ); // unmodified, AddConsoleAliasW just has bad const qualifications
    let mut target      = widen0(target     ); // unmodified, AddConsoleAliasW just has bad const qualifications
    let mut exe_name    = widen0(exe_name   ); // unmodified, AddConsoleAliasW just has bad const qualifications
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
    let mut source      = widen0(source     ); // unmodified, AddConsoleAliasW just has bad const qualifications
    let mut exe_name    = widen0(exe_name   ); // unmodified, AddConsoleAliasW just has bad const qualifications
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
    let mut source      = widen0(source     ); // unmodified, GetConsoleAliasW just has bad const qualifications
    let mut exe_name    = widen0(exe_name   ); // unmodified, GetConsoleAliasW just has bad const qualifications
    let bytes = wrap_last_error(|| unsafe { GetConsoleAliasW(source.as_mut_ptr(), target_buffer.as_mut_ptr(), size_of_val(target_buffer) as _, exe_name.as_mut_ptr()) })?;
    Ok(TextRef(strip0(&target_buffer[..(bytes/2) as _])))
}

/// \[[GetConsoleAliasW]\] Retrieves the text for the specified console alias and executable.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # use std::ffi::*;
/// # use std::os::windows::prelude::*;
/// # let _ = (|| -> std::io::Result<()> {
/// let alias : OsString = get_console_alias_os("test", "cmd.exe")?;
/// # Ok(())
/// # })();
/// ```
///
/// [GetConsoleAliasW]: https://docs.microsoft.com/en-us/windows/console/getconsolealias
pub fn get_console_alias_os(source: impl AsRef<OsStr>, exe_name: impl AsRef<OsStr>) -> io::Result<OsString> {
    let mut target_buffer = [0u16; 512];
    let mut source      = widen0(source     ); // unmodified, GetConsoleAliasW just has bad const qualifications
    let mut exe_name    = widen0(exe_name   ); // unmodified, GetConsoleAliasW just has bad const qualifications

    match wrap_last_error(|| unsafe { GetConsoleAliasW(source.as_mut_ptr(), target_buffer.as_mut_ptr(), size_of_val(&target_buffer) as _, exe_name.as_mut_ptr()) }) {
        Ok(bytes) => return Ok(wide0_to_os(&target_buffer[..(bytes/2) as _])),
        Err(err) if err.raw_os_error() == Some(ERROR_INSUFFICIENT_BUFFER as _) => {},
        Err(err) => return Err(err),
    }

    let mut target_buffer = vec![0u16; 0];
    loop {
        target_buffer.resize(target_buffer.capacity(), 0);
        match wrap_last_error(|| unsafe { GetConsoleAliasW(source.as_mut_ptr(), target_buffer.as_mut_ptr(), size_of_val(&target_buffer) as _, exe_name.as_mut_ptr()) }) {
            Ok(bytes) => return Ok(wide0_to_os(&target_buffer[..(bytes/2) as _])),
            Err(err) if err.raw_os_error() == Some(ERROR_INSUFFICIENT_BUFFER as _) => {},
            Err(err) => return Err(err),
        }
        target_buffer.push(0);
    }
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
    let mut exe_name    = widen0(exe_name); // unmodified, GetConsoleAliasesW just has bad const qualifications
    let bytes = wrap_last_error(|| unsafe { GetConsoleAliasesW(alias_buffer.as_mut_ptr(), size_of_val(alias_buffer) as _, exe_name.as_mut_ptr()) })?;
    Ok(TextNsvRef(&alias_buffer[..(bytes/2) as _]))
}

// TODO: get_console_aliases_vec? get_console_aliases_inplace?

/// \[[GetConsoleAliasesLengthW]\] Retrieves the required size for the buffer, **in bytes**, for use by the [get_console_aliases] function.
///
/// [GetConsoleAliasesLengthW]: https://docs.microsoft.com/en-us/windows/console/getconsolealiaseslength
pub fn get_console_aliases_length(exe_name: impl AsRef<OsStr>) -> LengthBytesOrWchars {
    let mut exe_name = widen0(exe_name); // unmodified, GetConsoleAliasesLengthW just has bad const qualifications
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
    let bytes = wrap_last_error(|| unsafe { GetConsoleAliasExesW(exe_name_buffer.as_mut_ptr(), size_of_val(exe_name_buffer) as _) })?;
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

    set_err_1(); clear_console_alias("test-alias1", (), exe).unwrap();
    set_err_1(); clear_console_alias("test-alias1", (), exe).unwrap(); // no error despite removing a removed alias
    set_err_1(); clear_console_alias("test-never",  (), exe).unwrap(); // no error despite removing a never-existing alias
    set_err_1(); clear_console_alias("test-alias2", (), exe).unwrap();
    set_err_1(); clear_console_alias("test=equal", (), exe).unwrap(); // keys containins `=`s are super gross

    set_err_1(); add_console_alias("test-alias1", "old", exe).unwrap();
    set_err_1(); add_console_alias("test-alias1", "alias1target", exe).unwrap(); // no error despite overwriting an alias
    set_err_1(); add_console_alias("test-alias2", "alias2target", exe).unwrap();
    set_err_1(); add_console_alias("test=equal", "value=value", exe).unwrap(); // keys containins `=`s are super gross

    set_err_1(); add_console_alias("test-removed", "temp", exe).unwrap();
    set_err_1(); clear_console_alias("test-removed", (), exe).unwrap();



    set_err_1();
    let err = get_console_alias("test-never",  &mut [0u16; 512], exe).unwrap_err();
    assert_eq!(err.raw_os_error(), Some(31));
    assert_eq!(err.raw_os_error(), Some(ERROR_GEN_FAILURE as _));
    assert_eq!(err.kind(), io::ErrorKind::Other);

    set_err_1();
    let err = get_console_alias("test-removed",  &mut [0u16; 512], exe).unwrap_err();
    assert_eq!(err.raw_os_error(), Some(31));
    assert_eq!(err.raw_os_error(), Some(ERROR_GEN_FAILURE as _));
    assert_eq!(err.kind(), io::ErrorKind::Other);

    set_err_1();
    let alias1 = get_console_alias("test-alias1", &mut [0u16; b"alias1target\0".len()], exe).unwrap().to_os_string();
    assert_eq!(alias1, "alias1target");

    set_err_1();
    let err = get_console_alias("test-alias1", &mut [0u16; b"alias1target".len()], exe).unwrap_err();
    assert_eq!(err.raw_os_error(), Some(122));
    assert_eq!(err.raw_os_error(), Some(ERROR_INSUFFICIENT_BUFFER as _));
    assert_eq!(err.kind(), io::ErrorKind::Other);

    set_err_1();
    let alias2 = get_console_alias("test-alias2", &mut [0u16; 512], exe).unwrap().to_os_string();
    assert_eq!(alias2, "alias2target");

    set_err_1();
    let equal = get_console_alias("test=equal", &mut [0u16; 512], exe).unwrap().to_os_string();
    assert_eq!(equal, "value=value");

    set_err_1(); get_console_alias("test", &mut [0u16; 512], exe).unwrap_err();
    set_err_1(); get_console_alias("test=equal", &mut [0u16; 512], exe).unwrap();
    set_err_1(); get_console_alias("test=equal=value", &mut [0u16; 512], exe).unwrap_err();



    set_err_1();
    let err = get_console_alias_os("test-never", exe).unwrap_err();
    assert_eq!(err.raw_os_error(), Some(31));
    assert_eq!(err.raw_os_error(), Some(ERROR_GEN_FAILURE as _));
    assert_eq!(err.kind(), io::ErrorKind::Other);

    set_err_1();
    let err = get_console_alias_os("test-removed", exe).unwrap_err();
    assert_eq!(err.raw_os_error(), Some(31));
    assert_eq!(err.raw_os_error(), Some(ERROR_GEN_FAILURE as _));
    assert_eq!(err.kind(), io::ErrorKind::Other);

    set_err_1();
    let alias1 = get_console_alias_os("test-alias1", exe).unwrap();
    assert_eq!(alias1, "alias1target");

    set_err_1();
    let alias2 = get_console_alias_os("test-alias2", exe).unwrap().to_os_string();
    assert_eq!(alias2, "alias2target");

    set_err_1();
    let equal = get_console_alias_os("test=equal", exe).unwrap().to_os_string();
    assert_eq!(equal, "value=value");

    set_err_1(); get_console_alias_os("test", exe).unwrap_err();
    set_err_1(); get_console_alias_os("test=equal", exe).unwrap();
    set_err_1(); get_console_alias_os("test=equal=value", exe).unwrap_err();



    set_err_1();
    let mut aliases = [0u16; 512];
    let mut aliases = get_console_aliases(&mut aliases, exe).unwrap().collect::<Vec<_>>();
    aliases.sort();
    assert_eq!(aliases.len(), 3);
    assert!(aliases[0].as_wchars() == wch!("test-alias1=alias1target"));
    assert!(aliases[1].as_wchars() == wch!("test-alias2=alias2target"));
    assert!(aliases[2].as_wchars() == wch!("test=equal=value=value"));

    set_err_1();
    assert_eq!(get_console_aliases_length(exe).wchars(), aliases.iter().map(|a| a.len()+1).sum());

    set_err_1();
    let mut exes = [0u16; 512];
    let exes = get_console_alias_exes(&mut exes).unwrap().collect::<Vec<_>>();
    assert!(exes.contains(&TextRef(wch!("maulingmonkey-console-winapi-wrappers-test.exe"))));

    set_err_1(); clear_console_alias("test-alias1", (), exe).unwrap();
    set_err_1(); clear_console_alias("test-alias2", (), exe).unwrap();
    set_err_1(); clear_console_alias("test=equal",  (), exe).unwrap();

    set_err_1(); assert!(get_console_aliases(&mut [0u16; 512], exe).unwrap().collect::<Vec<_>>().is_empty());
    set_err_1(); assert!(get_console_aliases(&mut [0u16; 512], bad_exe).unwrap().collect::<Vec<_>>().is_empty());

    fn set_err_1() { unsafe { SetLastError(1) }; }
}

/// Workaround errata in several console functions:
///
/// *   [`GetConsoleAlias`]
/// *   [`GetConsoleAliases`]
/// *   [`GetConsoleAliasExes`]
///
/// These functions are **mistakenly** documented as returning zero on failure, nonzero (number of bytes read out) on success.
/// However, in the case of an insufficiently large buffer, they return **nonzero** as if succeeding - or more
/// specifically, they return the size of the too-small buffer - without populating the contents of the buffer at all.
///
/// Additionally, merely checking the last set error is insufficient, since these functions will not *clear* the last
/// set error should the function succeed.  As such, this function:
///
/// 1.  Clears the last set error before calling `f()`, such that it can be reliably queried.
/// 2.  Checks the last set error after calling `f()`, regardless of the function's return value.
///
/// Anything other than [`ERROR_SUCCESS`] is converted into an error.
fn wrap_last_error<R>(f: impl FnOnce() -> R) -> io::Result<R> {
    unsafe { SetLastError(0) };
    let r = f();
    last_os_error_unless_success()?;
    Ok(r)
}

fn last_os_error_unless_success() -> io::Result<()> {
    let err = io::Error::last_os_error();
    match err.raw_os_error() {
        Some(0) => Ok(()),
        _ => Err(err),
    }
}

/// Remove a final `\0` if present.
fn strip0(s: &[u16]) -> &[u16] {
    s.strip_suffix(&[0]).unwrap_or(s)
}

/// Encode as UTF16 and `\0` terminate
fn widen0(s: impl AsRef<OsStr>) -> Vec<u16> {
    s.as_ref().encode_wide().chain(Some(0)).collect()
}

/// Take a `\0`-terminated, UTF16-ish string, and convert it into an [`OsString`].
fn wide0_to_os(s: impl AsRef<[u16]>) -> OsString {
    OsString::from_wide(strip0(s.as_ref()))
}
