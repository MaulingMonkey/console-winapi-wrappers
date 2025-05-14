use bytemuck::{Pod, Zeroable};



#[doc(alias = "CHAR_INFO::Char")]
#[doc(alias = "KEY_EVENT_RECORD::uChar")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/char-info-str)\]
/// union { ascii_char: u8, unicode_char: u16 }
///
/// I strongly recommend treating this as a [`u16`] or [`wchar_t`](https://learn.microsoft.com/en-us/cpp/cpp/char-wchar-t-char16-t-char32-t).
/// That Win32 made this a union in the first place is arguably a mistake.
///
/// This wrapper type:
/// -   Always 0-initializes the unused byte for ASCII based stuff.
/// -   Assumes any code accessed via FFI interop does similar (assumably true for Win32 API calls, less true for 3rd party FFI.)
/// -   Deprecates and otherwise discourages actually using the provided ASCII based accessors.
///
/// Aside from the [somewhat questionable business](https://stackoverflow.com/questions/11373203/accessing-inactive-union-member-and-undefined-behavior)
/// of type punning with unions in the first place, [especially with mismatched value sizes](https://www.reddit.com/r/C_Programming/comments/qvbsqa/type_punning_with_union/),
/// the behavior of using the ASCII variant on big endian systems is utterly bogus (it will presuambly access the *first byte* for AsciiChar, rather than the *least significant byte*:)
///
/// ```cpp
/// CHAR_INFO ci;
///
/// // will fail on big endian:
/// ci.Char.UnicodeChar = L'!'; // U+0021
/// assert(ci.Char.AsciiChar == '!');
///
/// // the miserable behavior of big endian:
/// ci.Char.UnicodeChar = L'℀'; // U+2100
/// assert(ci.Char.AsciiChar == '!');
///
/// // may fail on all platforms due to uninitialized bytes:
/// ci.Char.AsciiChar = '!';
/// assert(ci.Char.UnicodeChar == L'!');
/// ```
///
#[derive(Clone, Copy, Pod, Default, Zeroable, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct AsciiOrUnicodeChar(u16);

impl AsciiOrUnicodeChar {
    #[deprecated = "prefer unicode / u16 based code: the behavior of this call on big endian systems is unlikely to be what you want."]
    pub const fn from_ascii_char(ascii: u8) -> Self {
        Self(u16::from_ne_bytes([ascii, 0]))
    }

    pub const fn from_unicode_char(unicode: u16) -> Self {
        Self(unicode)
    }

    pub const fn from_char(unicode: char) -> (Option<Self>, Self) {
        let unicode = unicode as u32;
        let Some(u_) = unicode.checked_sub(0x10000) else { return (None, Self(unicode as _)) };
        let hi_surrogate = 0xD800 | ((u_ >> 10) & 0x3FF);
        let lo_surrogate = 0xDC00 | ((u_ >>  0) & 0x3FF);
        (Some(Self(hi_surrogate as _)), Self(lo_surrogate as _))
    }

    #[deprecated = "prefer unicode / u16 based code: the behavior of this call on big endian systems is unlikely to be what you want."]
    pub const fn ascii_char(self) -> u8 {
        let [ascii, _] = self.0.to_ne_bytes();
        ascii
    }

    pub const fn unicode_char(self) -> u16 {
        self.0
    }
}

//pl From<u8 > for AsciiOrUnicodeChar { fn from(value: u8 ) -> Self { Self::from_ascii_char  (value) } } // uses deprecated functionality
impl From<u16> for AsciiOrUnicodeChar { fn from(value: u16) -> Self { Self::from_unicode_char(value) } }
//pl From<AsciiOrUnicodeChar> for u8  { fn from(value: AsciiOrUnicodeChar) -> Self { value.ascii_char() } } // lossy, intentionally omitted
impl From<AsciiOrUnicodeChar> for u16 { fn from(value: AsciiOrUnicodeChar) -> Self { value.unicode_char() } }

impl TryFrom<char> for AsciiOrUnicodeChar {
    type Error = [Self; 2];
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match Self::from_char(value) {
            (None, lo)      => Ok(lo),
            (Some(hi), lo)  => Err([hi, lo]),
        }
    }
}

impl core::fmt::Debug for AsciiOrUnicodeChar {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        let unit = self.unicode_char();
        if let Ok(ch) = char::try_from(u32::from(unit)) {
            core::fmt::Debug::fmt(&ch, fmt)
        } else {
            write!(fmt, "'\\u{{{unit:04X}}}'")
        }
    }
}
