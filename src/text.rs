use std::convert::*;
use std::ffi::*;
use std::fmt::{self, Debug, Formatter};
use std::ops::*;
use std::os::windows::prelude::*;



/// \[<strike>microsoft.com</strike>\]
/// A **ref**erence to wide **text**.
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)] pub struct TextRef<'a>(pub(crate) &'a [u16]);

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



/// \[<strike>microsoft.com</strike>\]
/// A **ref**erence to wide **text**, as **n**ull **s**eparated **v**alues.
/// <br>
/// (impl [Iterator]<Item = [TextRef]>)
///
#[derive(Clone, Copy, Debug)] pub struct TextNsvRef<'a>(pub(crate) &'a [u16]);

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



/// \[<strike>microsoft.com</strike>\]
/// Length of wide text, disambiguated as [bytes](Self::bytes) or [wchars](Self::wchars).
///
/// Various console functions return lengths for wide text buffers in bytes / [`u8`]s, despite the fact that you probably
/// want and need lengths in `wchar_t`s / [`u16`]s.  This type makes things more explicit by forcing you to choose.
///
/// (Functions which return the lengths of *narrow* should instead return [`usize`], as there is no ambiguity.)
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)] pub struct TextLength(pub(crate) usize);

impl TextLength {
    pub fn bytes(&self) -> usize { self.0 }
    pub fn wchars(&self) -> usize { (self.0/2) + (self.0&1) }
}
