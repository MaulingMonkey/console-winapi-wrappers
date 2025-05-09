use crate::*;

use bytemuck::{Pod, Zeroable};

use winapi::um::wincontypes::CHAR_INFO;



#[doc(alias = "CHAR_INFO")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/char-info-str)\]
/// struct { char: [AsciiOrUnicodeChar], attributes: [Attributes] }
///
#[derive(Clone, Copy, Pod, Default, Zeroable, Debug, PartialEq, Eq)] // PartialOrd, Ord, Hash?
#[repr(C)] pub struct CharInfo {
    pub char:           AsciiOrUnicodeChar,
    pub attributes:     Attributes,
}

impl CharInfo {
    pub const fn new(char: u16, attributes: Attributes) -> Self {
        Self {
            char: AsciiOrUnicodeChar::from_unicode_char(char),
            attributes
        }
    }
}

impl From<CharInfo> for CHAR_INFO { fn from(value: CharInfo ) -> Self { unsafe { core::mem::transmute(value) } } }
impl From<CHAR_INFO> for CharInfo { fn from(value: CHAR_INFO) -> Self { unsafe { core::mem::transmute(value) } } }

const _ : () = {
    use core::mem::offset_of;
    assert!(align_of::<CHAR_INFO>() == align_of::<CharInfo>());
    assert!(size_of ::<CHAR_INFO>() == size_of ::<CharInfo>());
    assert!(offset_of!(CharInfo, char) == offset_of!(CHAR_INFO, Char));
};

#[test] fn layout() {
    let a = CharInfo::new(0x1234, Zeroable::zeroed());
    let b : CHAR_INFO = a.into();
    assert_eq!(a.char.ascii_char(),     unsafe { *b.Char.AsciiChar() } as u8);
    assert_eq!(a.char.unicode_char(),   unsafe { *b.Char.UnicodeChar() });
}
