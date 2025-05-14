use crate::Coord;
use winapi::um::wincon::CONSOLE_FONT_INFO;



#[doc(alias = "CONSOLE_FONT_INFO")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/console-font-info-str)\]
///
#[derive(Clone, Copy, bytemuck::Pod, Default, bytemuck::Zeroable, Debug)] // PartialEq, Eq, PartialOrd, Ord, Hash?
#[repr(C)] pub struct ConsoleFontInfo {
    pub font:       u32,
    pub font_size:  Coord,
}

impl From<CONSOLE_FONT_INFO> for ConsoleFontInfo { fn from(value: CONSOLE_FONT_INFO ) -> Self { unsafe { core::mem::transmute(value) } } }
impl From<ConsoleFontInfo> for CONSOLE_FONT_INFO { fn from(value: ConsoleFontInfo   ) -> Self { unsafe { core::mem::transmute(value) } } }

const _ : () = {
    use core::mem::offset_of;
    assert!(align_of::<CONSOLE_FONT_INFO>() == align_of::<ConsoleFontInfo>());
    assert!(size_of ::<CONSOLE_FONT_INFO>() == size_of ::<ConsoleFontInfo>());
    assert!(offset_of!(ConsoleFontInfo, font                ) == offset_of!(CONSOLE_FONT_INFO, nFont               ));
    assert!(offset_of!(ConsoleFontInfo, font_size           ) == offset_of!(CONSOLE_FONT_INFO, dwFontSize          ));
};
