use crate::Coord;
use winapi::um::wincon::CONSOLE_FONT_INFOEX;
use winapi::um::wingdi::LF_FACESIZE;



#[doc(alias = "CONSOLE_FONT_INFOEX")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/console-font-infoex)\]
///
#[derive(Clone, Copy, Default, Debug)] // PartialEq, Eq, PartialOrd, Ord, Hash?
#[repr(C)] pub struct ConsoleFontInfoEx {
    pub self_size:      u32,
    pub font:           u32,
    pub font_size:      Coord,
    pub font_family:    u32,
    pub font_weight:    u32,
    pub face_name:      abistr::CStrBuf<u16, LF_FACESIZE>, // ≈ [wchar_t; 32]

    #[doc(hidden)]
    #[deprecated = "do not directly use this field, instead initialize with e.g. `.. bytemuck::zeroed()`"]
    pub _non_exhaustive:        (),
}

unsafe impl bytemuck::Pod       for ConsoleFontInfoEx {}
unsafe impl bytemuck::Zeroable  for ConsoleFontInfoEx {}
const _ : () = assert!(size_of::<ConsoleFontInfoEx>() < size_of::<Option<ConsoleFontInfoEx>>(), "unexpected niche available in `ConsoleFontInfoEx`, it's probably not `bytemuck::Pod`");

impl From<CONSOLE_FONT_INFOEX> for ConsoleFontInfoEx {
    fn from(value: CONSOLE_FONT_INFOEX) -> Self {
        #[allow(deprecated)] Self {
            self_size:              value.cbSize,
            font:                   value.nFont,
            font_size:              value.dwFontSize.into(),
            font_family:            value.FontFamily,
            font_weight:            value.FontWeight,
            face_name:              unsafe { core::mem::transmute(value.FaceName) }, // XXX
            _non_exhaustive:        (),
        }
    }
}

impl From<ConsoleFontInfoEx> for CONSOLE_FONT_INFOEX {
    fn from(value: ConsoleFontInfoEx) -> Self {
        Self {
            cbSize:                 value.self_size,
            nFont:                  value.font,
            dwFontSize:             value.font_size.into(),
            FontFamily:             value.font_family,
            FontWeight:             value.font_weight,
            FaceName:               unsafe { core::mem::transmute(value.face_name) }, // XXX
            .. Default::default()
        }
    }
}

const _ : () = {
    use core::mem::offset_of;

    assert!(size_of ::<CONSOLE_FONT_INFOEX>() >= size_of ::<ConsoleFontInfoEx>());
    if      size_of ::<CONSOLE_FONT_INFOEX>() == size_of ::<ConsoleFontInfoEx>() {
        assert!(align_of::<CONSOLE_FONT_INFOEX>() == align_of::<ConsoleFontInfoEx>());
    }

    assert!(offset_of!(ConsoleFontInfoEx, self_size           ) == offset_of!(CONSOLE_FONT_INFOEX, cbSize              ));
    assert!(offset_of!(ConsoleFontInfoEx, font                ) == offset_of!(CONSOLE_FONT_INFOEX, nFont               ));
    assert!(offset_of!(ConsoleFontInfoEx, font_size           ) == offset_of!(CONSOLE_FONT_INFOEX, dwFontSize          ));
    assert!(offset_of!(ConsoleFontInfoEx, font_family         ) == offset_of!(CONSOLE_FONT_INFOEX, FontFamily          ));
    assert!(offset_of!(ConsoleFontInfoEx, font_weight         ) == offset_of!(CONSOLE_FONT_INFOEX, FontWeight          ));
    assert!(offset_of!(ConsoleFontInfoEx, face_name           ) == offset_of!(CONSOLE_FONT_INFOEX, FaceName            ));
};

#[test] fn layout() {
    assert_eq!(size_of::<CONSOLE_FONT_INFOEX>(), size_of::<ConsoleFontInfoEx>(), "`CONSOLE_FONT_INFOEX` has gained new fields, update `ConsoleFontInfoEx`");
}
