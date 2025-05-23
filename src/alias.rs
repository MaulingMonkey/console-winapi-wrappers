use crate::*;

use winapi::shared::winerror::*;
use winapi::um::errhandlingapi::*;
use winapi::um::wincon::*;

use std::ffi::*;
use std::io;
use std::os::windows::prelude::*;

use core::mem::size_of_val;
use core::ops::*;
use core::ptr::*;



#[doc(alias = "AddConsoleAlias")]
#[doc(alias = "AddConsoleAliasW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/addconsolealias)\]
/// Defines a console alias for the specified executable.
///
/// ### Example
/// ```no_run
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # let _ = (|| -> std::io::Result<()> {
/// // https://learn.microsoft.com/en-us/windows/console/console-aliases
/// add_console_alias("test", r"cd \<a_very_long_path>\test", "cmd.exe")?;
/// # Ok(())
/// # })();
/// ```
///
pub fn add_console_alias(source: impl AsRef<OsStr>, target: impl AsRef<OsStr>, exe_name: impl AsRef<OsStr>) -> io::Result<()> {
    let mut source      = widen0(source     ); // unmodified, AddConsoleAliasW just has bad const qualifications
    let mut target      = widen0(target     ); // unmodified, AddConsoleAliasW just has bad const qualifications
    let mut exe_name    = widen0(exe_name   ); // unmodified, AddConsoleAliasW just has bad const qualifications
    succeeded_to_result(unsafe { AddConsoleAliasW(source.as_mut_ptr(), target.as_mut_ptr(), exe_name.as_mut_ptr()) })
}

#[doc(alias = "AddConsoleAlias")]
#[doc(alias = "AddConsoleAliasW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/addconsolealias)\]
/// Clears a console alias for the specified executable.
///
/// ### Example
/// ```no_run
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # let _ = (|| -> std::io::Result<()> {
/// // https://learn.microsoft.com/en-us/windows/console/console-aliases
/// clear_console_alias("test", (), "cmd.exe")?;
/// # Ok(())
/// # })();
/// ```
///
pub fn clear_console_alias(source: impl AsRef<OsStr>, _target: (), exe_name: impl AsRef<OsStr>) -> io::Result<()> {
    let mut source      = widen0(source     ); // unmodified, AddConsoleAliasW just has bad const qualifications
    let mut exe_name    = widen0(exe_name   ); // unmodified, AddConsoleAliasW just has bad const qualifications
    succeeded_to_result(unsafe { AddConsoleAliasW(source.as_mut_ptr(), null_mut(), exe_name.as_mut_ptr()) })
}



#[doc(alias = "GetConsoleAlias")]
#[doc(alias = "GetConsoleAliasW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/getconsolealias)\]
/// Retrieves the text for the specified console alias and executable.
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
pub fn get_console_alias<'t>(source: impl AsRef<OsStr>, target_buffer: &'t mut impl AsMut<[u16]>, exe_name: impl AsRef<OsStr>) -> io::Result<TextRef<'t>> {
    let target_buffer   = target_buffer .as_mut();
    let mut source      = widen0(source     ); // unmodified, GetConsoleAliasW just has bad const qualifications
    let mut exe_name    = widen0(exe_name   ); // unmodified, GetConsoleAliasW just has bad const qualifications

    // SAFETY: yes, `GetConsoleAliasW` expects a buffer size in *bytes*, not *units*
    let bytes = wrap_last_error(|| unsafe { GetConsoleAliasW(source.as_mut_ptr(), target_buffer.as_mut_ptr(), u32::try_from(size_of_val(target_buffer)).unwrap_or(!1), exe_name.as_mut_ptr()) })?;
    Ok(TextRef(strip0(&target_buffer[..(bytes/2) as _])))
}

#[doc(alias = "GetConsoleAlias")]
#[doc(alias = "GetConsoleAliasW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/getconsolealias)\]
/// Retrieves the text for the specified console alias and executable.
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
pub fn get_console_alias_os(source: impl AsRef<OsStr>, exe_name: impl AsRef<OsStr>) -> io::Result<OsString> {
    let mut target_buffer = [0u16; 512];
    let mut source      = widen0(source     ); // unmodified, GetConsoleAliasW just has bad const qualifications
    let mut exe_name    = widen0(exe_name   ); // unmodified, GetConsoleAliasW just has bad const qualifications

    // SAFETY: yes, `GetConsoleAliasW` expects a buffer size in *bytes*, not *units*
    match wrap_last_error(|| unsafe { GetConsoleAliasW(source.as_mut_ptr(), target_buffer.as_mut_ptr(), size_of_val_32_sized(&target_buffer), exe_name.as_mut_ptr()) }) {
        Ok(bytes) => return Ok(wide0_to_os(&target_buffer[..(bytes/2) as _])),
        Err(err) if err.raw_os_error() == Some(ERROR_INSUFFICIENT_BUFFER as _) => {},
        Err(err) => return Err(err),
    }

    let mut target_buffer = vec![0u16; 0];
    loop {
        target_buffer.resize(target_buffer.capacity(), 0);

        // SAFETY: yes, `GetConsoleAliasW` expects a buffer size in *bytes*, not *units*
        match wrap_last_error(|| unsafe { GetConsoleAliasW(source.as_mut_ptr(), target_buffer.as_mut_ptr(), u32::try_from(size_of_val(&target_buffer[..])).unwrap_or(!1), exe_name.as_mut_ptr()) }) {
            Ok(bytes) => return Ok(wide0_to_os(&target_buffer[..(bytes/2) as _])),
            Err(err) if err.raw_os_error() == Some(ERROR_INSUFFICIENT_BUFFER as _) => {},
            Err(err) => return Err(err),
        }
        target_buffer.push(0);
    }
}



#[doc(alias = "GetConsoleAliases")]
#[doc(alias = "GetConsoleAliasesW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/getconsolealiases)\]
/// Retrieves all defined console aliases for the specified executable.
///
/// Separates keys and values with `=`.  As keys can also contain `=`s, this can result in ambiguous aliases!  Given:
///
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// dbg!(get_console_aliases(&mut [0u16; 512], "cmd.exe").unwrap().collect::<Vec<_>>());
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
/// let mut aliases = vec![0u16; get_console_aliases_length(exe).wchars_ceil()];
/// for alias in get_console_aliases(&mut aliases, exe)? {
///     dbg!(alias.to_os_string());
/// }
/// # Ok(())
/// # })();
/// ```
///
pub fn get_console_aliases<'t>(alias_buffer: &'t mut impl AsMut<[u16]>, exe_name: impl AsRef<OsStr>) -> io::Result<TextNsvRef<'t>> {
    let alias_buffer    = alias_buffer.as_mut();
    let mut exe_name    = widen0(exe_name); // unmodified, GetConsoleAliasesW just has bad const qualifications
    unsafe { get_console_aliases_impl(alias_buffer, &mut exe_name) }
}

/// ### Safety
/// *   `exe_name` should be `\0` terminated
unsafe fn get_console_aliases_impl<'t>(alias_buffer: &'t mut [u16], exe_name: &mut [u16]) -> io::Result<TextNsvRef<'t>> {
    debug_assert!(exe_name.ends_with(&[0]));

    // SAFETY: yes, `GetConsoleAliasW` expects a buffer size in *bytes*, not *units*
    let bytes = wrap_last_error(|| unsafe { GetConsoleAliasesW(alias_buffer.as_mut_ptr(), u32::try_from(size_of_val(alias_buffer)).unwrap_or(!1), exe_name.as_mut_ptr()) })?;
    Ok(TextNsvRef(&alias_buffer[..(bytes/2) as _]))
}

#[doc(alias = "GetConsoleAliases")]
#[doc(alias = "GetConsoleAliasesW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/getconsolealiases)\]
/// Retrieves all defined console aliases for the specified executable.
///
/// Separates keys and values with `=`.  As keys can also contain `=`s, this can result in ambiguous aliases!  Given:
///
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// dbg!(get_console_aliases_os("cmd.exe").unwrap().collect::<Vec<_>>());
/// ```
///
/// ```text
/// [src\alias.rs:???] get_console_aliases(&mut [0u16; 512], exe).unwrap().collect::<Vec<_>>() = [
///     "test-alias1=alias1target",
///     "test-alias2=alias2target",
///     "test=equal=value=value",
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
/// for alias in get_console_aliases_os("cmd.exe")? {
///     dbg!(alias);
/// }
/// # Ok(())
/// # })();
/// ```
///
pub fn get_console_aliases_os<'t>(exe_name: impl AsRef<OsStr>) -> io::Result<impl Iterator<Item = OsString>> {
    let mut exe_name    = widen0(exe_name); // unmodified, GetConsoleAliasesW just has bad const qualifications

    let mut buf = vec![0u16; unsafe { get_console_aliases_length_impl(&mut exe_name) }.wchars_ceil()];
    loop {
        buf.resize(buf.capacity(), 0);
        match unsafe { get_console_aliases_impl(&mut buf, &mut exe_name) } {
            Err(err) if err.raw_os_error() == Some(ERROR_INSUFFICIENT_BUFFER as _) => {}, // race condition: aliases grown between fetching length and fetching aliases?
            Err(err)    => return Err(err),
            Ok(nsv)     => return Ok(nsv.map(|v| v.to_os_string()).collect::<Vec<_>>().into_iter()),
        }
        buf.push(0);
    }
}

#[doc(alias = "GetConsoleAliasesLength")]
#[doc(alias = "GetConsoleAliasesLengthW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/getconsolealiaseslength)\]
/// Retrieves the buffer size required for [get_console_aliases].
///
pub fn get_console_aliases_length(exe_name: impl AsRef<OsStr>) -> TextLength {
    unsafe { get_console_aliases_length_impl(&mut widen0(exe_name)) }
}

/// ### Safety
/// *   `exe_name` should be `\0` terminated
unsafe fn get_console_aliases_length_impl(exe_name: &mut [u16]) -> TextLength {
    debug_assert!(exe_name.ends_with(&[0]));
    TextLength(unsafe { GetConsoleAliasesLengthW(exe_name.as_mut_ptr()) } as _)
}



#[doc(alias = "GetConsoleAliasExes")]
#[doc(alias = "GetConsoleAliasExesW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/getconsolealiasexes)\]
/// Retrieves the names of all executable files with console aliases defined.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # let _ = (|| -> std::io::Result<()> {
/// let mut exes = vec![0u16; get_console_alias_exes_length().wchars_ceil()];
/// for exe in get_console_alias_exes(&mut exes)? {
///     dbg!(exe.to_os_string());
/// }
/// # Ok(())
/// # })();
/// ```
///
pub fn get_console_alias_exes(exe_name_buffer: &mut impl AsMut<[u16]>) -> io::Result<TextNsvRef> {
    let exe_name_buffer = exe_name_buffer.as_mut();

    // SAFETY: yes, `GetConsoleAliasW` expects a buffer size in *bytes*, not *units*
    let bytes = wrap_last_error(|| unsafe { GetConsoleAliasExesW(exe_name_buffer.as_mut_ptr(), u32::try_from(size_of_val(exe_name_buffer)).unwrap_or(!1)) })?;
    Ok(TextNsvRef(&exe_name_buffer[..(bytes/2) as _]))
}

#[doc(alias = "GetConsoleAliasExes")]
#[doc(alias = "GetConsoleAliasExesW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/getconsolealiasexes)\]
/// Retrieves the names of all executable files with console aliases defined.
///
/// ### Example
/// ```
/// # use maulingmonkey_console_winapi_wrappers::*;
/// # let _ = (|| -> std::io::Result<()> {
/// for exe in get_console_alias_exes_os()? {
///     dbg!(exe.to_os_string());
/// }
/// # Ok(())
/// # })();
/// ```
///
pub fn get_console_alias_exes_os() -> io::Result<impl Iterator<Item = OsString>> {
    let mut buf = vec![0u16; get_console_alias_exes_length().wchars_ceil()];
    loop {
        buf.resize(buf.capacity(), 0);
        match get_console_alias_exes(&mut buf) {
            Err(err) if err.raw_os_error() == Some(ERROR_INSUFFICIENT_BUFFER as _) => {}, // race condition: exes grown between fetching length and fetching exes?
            Err(err)    => return Err(err),
            Ok(nsv)     => return Ok(nsv.map(|v| v.to_os_string()).collect::<Vec<_>>().into_iter()),
        }
        buf.push(0);
    }
}

#[doc(alias = "GetConsoleAliasExesLength")]
#[doc(alias = "GetConsoleAliasExesLengthW")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/getconsolealiasexeslength)\]
/// Retrieves the buffer size required for [get_console_alias_exes].
///
pub fn get_console_alias_exes_length() -> TextLength {
    TextLength(unsafe { GetConsoleAliasExesLengthW() as _ })
}



#[test] fn aliases() {
    use wchar::wch;

    let exe = "maulingmonkey-console-winapi-wrappers-test.exe";
    let exe2 = "maulingmonkey-console-winapi-wrappers-test-2.exe";
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

    set_err_1(); add_console_alias("test-alias3", "alias3target", exe2).unwrap();



    set_err_1();
    let err = get_console_alias("test-never",  &mut [0u16; 512], exe).unwrap_err();
    assert_eq!(err.raw_os_error(), Some(31));
    assert_eq!(err.raw_os_error(), Some(ERROR_GEN_FAILURE as _));
    //assert_eq!(err.kind(), io::ErrorKind::Other); // `std` behavior has changed: returns `Uncategorized` (unstable value) as of 1.86.0

    set_err_1();
    let err = get_console_alias("test-removed",  &mut [0u16; 512], exe).unwrap_err();
    assert_eq!(err.raw_os_error(), Some(31));
    assert_eq!(err.raw_os_error(), Some(ERROR_GEN_FAILURE as _));
    //assert_eq!(err.kind(), io::ErrorKind::Other); // `std` behavior has changed: returns `Uncategorized` (unstable value) as of 1.86.0

    set_err_1();
    let alias1 = get_console_alias("test-alias1", &mut [0u16; b"alias1target\0".len()], exe).unwrap().to_os_string();
    assert_eq!(alias1, "alias1target");

    set_err_1();
    let err = get_console_alias("test-alias1", &mut [0u16; b"alias1target".len()], exe).unwrap_err();
    assert_eq!(err.raw_os_error(), Some(122));
    assert_eq!(err.raw_os_error(), Some(ERROR_INSUFFICIENT_BUFFER as _));
    //assert_eq!(err.kind(), io::ErrorKind::Other); // `std` behavior has changed: returns `Uncategorized` (unstable value) as of 1.86.0

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
    //assert_eq!(err.kind(), io::ErrorKind::Other); // `std` behavior has changed: returns `Uncategorized` (unstable value) as of 1.86.0

    set_err_1();
    let err = get_console_alias_os("test-removed", exe).unwrap_err();
    assert_eq!(err.raw_os_error(), Some(31));
    assert_eq!(err.raw_os_error(), Some(ERROR_GEN_FAILURE as _));
    //assert_eq!(err.kind(), io::ErrorKind::Other); // `std` behavior has changed: returns `Uncategorized` (unstable value) as of 1.86.0

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
    let mut aliases = get_console_aliases_os(exe).unwrap().collect::<Vec<_>>();
    aliases.sort();
    assert_eq!(aliases.len(), 3);
    assert_eq!(aliases[0], "test-alias1=alias1target");
    assert_eq!(aliases[1], "test-alias2=alias2target");
    assert_eq!(aliases[2], "test=equal=value=value");

    set_err_1();
    assert_eq!(get_console_aliases_length(exe).wchars_floor(), aliases.iter().map(|a| a.len()+1).sum());

    set_err_1();
    let mut exes = [0u16; 512];
    let exes = get_console_alias_exes(&mut exes).unwrap().collect::<Vec<_>>();
    assert!(exes.contains(&TextRef(wch!("maulingmonkey-console-winapi-wrappers-test.exe"))));
    assert!(exes.contains(&TextRef(wch!("maulingmonkey-console-winapi-wrappers-test-2.exe"))));

    set_err_1();
    let exes = get_console_alias_exes_os().unwrap().collect::<Vec<_>>();
    assert!(exes.contains(&OsString::from("maulingmonkey-console-winapi-wrappers-test.exe")));
    assert!(exes.contains(&OsString::from("maulingmonkey-console-winapi-wrappers-test-2.exe")));

    set_err_1(); clear_console_alias("test-alias1", (), exe).unwrap();
    set_err_1(); clear_console_alias("test-alias2", (), exe).unwrap();
    set_err_1(); clear_console_alias("test-alias3", (), exe2).unwrap();
    set_err_1(); clear_console_alias("test=equal",  (), exe).unwrap();

    set_err_1(); assert!(get_console_aliases(&mut [0u16; 512], exe).unwrap().collect::<Vec<_>>().is_empty());
    set_err_1(); assert!(get_console_aliases(&mut [0u16; 512], exe2).unwrap().collect::<Vec<_>>().is_empty());
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
///
/// [`GetConsoleAlias`]:        https://learn.microsoft.com/en-us/windows/console/getconsolealias
/// [`GetConsoleAliases`]:      https://learn.microsoft.com/en-us/windows/console/getconsolealiases
/// [`GetConsoleAliasExes`]:    https://learn.microsoft.com/en-us/windows/console/getconsolealiasexes
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
