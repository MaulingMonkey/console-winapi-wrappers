use bytemuck::{Pod, Zeroable};

use core::convert::TryFrom;



#[doc(alias = "CHAR_INFO::Char")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/char-info-str)\]
/// union { ascii_char: u8, unicode_char: u16 }
///
#[derive(Clone, Copy, Pod, Default, Zeroable, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct AsciiOrUnicodeChar(u16);

impl AsciiOrUnicodeChar {
    pub const fn from_ascii_char(ascii: u8) -> Self {
        Self(u16::from_ne_bytes([ascii, 0]))
    }

    pub const fn from_unicode_char(unicode: u16) -> Self {
        Self(unicode)
    }

    pub const fn ascii_char(self) -> u8 {
        let [ascii, _] = self.0.to_ne_bytes();
        ascii
    }

    pub const fn unicode_char(self) -> u16 {
        self.0
    }
}

impl From<u8 > for AsciiOrUnicodeChar { fn from(value: u8 ) -> Self { Self::from_ascii_char  (value) } }
impl From<u16> for AsciiOrUnicodeChar { fn from(value: u16) -> Self { Self::from_unicode_char(value) } }
//pl From<AsciiOrUnicodeChar> for u8  { fn from(value: AsciiOrUnicodeChar) -> Self { value.ascii_char() } } // lossy, intentionally omitted
impl From<AsciiOrUnicodeChar> for u16 { fn from(value: AsciiOrUnicodeChar) -> Self { value.unicode_char() } }

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
