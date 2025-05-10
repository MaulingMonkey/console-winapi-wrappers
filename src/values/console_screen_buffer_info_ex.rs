use crate::{Attributes, ColorRef, Coord, SmallRect};
use winapi::um::wincon::CONSOLE_SCREEN_BUFFER_INFOEX;



#[doc(alias = "CONSOLE_SCREEN_BUFFER_INFOEX")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/console-screen-buffer-infoex)\]
///
#[derive(Clone, Copy, bytemuck::Pod, Default, bytemuck::Zeroable, Debug)] // PartialEq, Eq, PartialOrd, Ord, Hash?
#[repr(C)] pub struct ConsoleScreenBufferInfoEx {
    pub self_size:              u32,
    pub size:                   Coord,
    pub cursor_position:        Coord,
    pub attributes:             Attributes,
    pub window:                 SmallRect,
    pub maximum_window_size:    Coord,
    pub popup_attributes:       Attributes, // ???
    pub fullscreen_supported:   abibool::bool32,
    pub color_table:            [ColorRef; 16],

    #[doc(hidden)]
    #[deprecated = "do not directly use this field, instead initialize with e.g. `.. bytemuck::zeroed()`"]
    pub _non_exhaustive:        (),
}

impl From<CONSOLE_SCREEN_BUFFER_INFOEX> for ConsoleScreenBufferInfoEx {
    fn from(value: CONSOLE_SCREEN_BUFFER_INFOEX) -> Self {
        #[allow(deprecated)] Self {
            self_size:              value.cbSize,
            size:                   value.dwSize.into(),
            cursor_position:        value.dwCursorPosition.into(),
            attributes:             value.wAttributes.into(),
            window:                 value.srWindow.into(),
            maximum_window_size:    value.dwMaximumWindowSize.into(),
            popup_attributes:       value.wPopupAttributes.into(),
            fullscreen_supported:   value.bFullscreenSupported.into(),
            color_table:            value.ColorTable.map(|c| c.into()),
            _non_exhaustive:        (),
        }
    }
}

impl From<ConsoleScreenBufferInfoEx> for CONSOLE_SCREEN_BUFFER_INFOEX {
    fn from(value: ConsoleScreenBufferInfoEx) -> Self {
        Self {
            cbSize:                 value.self_size,
            dwSize:                 value.size.into(),
            dwCursorPosition:       value.cursor_position.into(),
            wAttributes:            value.attributes.into(),
            srWindow:               value.window.into(),
            dwMaximumWindowSize:    value.maximum_window_size.into(),
            wPopupAttributes:       value.popup_attributes.into(),
            bFullscreenSupported:   value.fullscreen_supported.into(),
            ColorTable:             value.color_table.map(|c| c.into()),
            .. Default::default()
        }
    }
}

const _ : () = {
    use core::mem::offset_of;

    assert!(size_of ::<CONSOLE_SCREEN_BUFFER_INFOEX>() >= size_of ::<ConsoleScreenBufferInfoEx>());
    if      size_of ::<CONSOLE_SCREEN_BUFFER_INFOEX>() == size_of ::<ConsoleScreenBufferInfoEx>() {
        assert!(align_of::<CONSOLE_SCREEN_BUFFER_INFOEX>() == align_of::<ConsoleScreenBufferInfoEx>());
    }

    assert!(offset_of!(ConsoleScreenBufferInfoEx, self_size             ) == offset_of!(CONSOLE_SCREEN_BUFFER_INFOEX, cbSize                ));
    assert!(offset_of!(ConsoleScreenBufferInfoEx, size                  ) == offset_of!(CONSOLE_SCREEN_BUFFER_INFOEX, dwSize                ));
    assert!(offset_of!(ConsoleScreenBufferInfoEx, cursor_position       ) == offset_of!(CONSOLE_SCREEN_BUFFER_INFOEX, dwCursorPosition      ));
    assert!(offset_of!(ConsoleScreenBufferInfoEx, attributes            ) == offset_of!(CONSOLE_SCREEN_BUFFER_INFOEX, wAttributes           ));
    assert!(offset_of!(ConsoleScreenBufferInfoEx, window                ) == offset_of!(CONSOLE_SCREEN_BUFFER_INFOEX, srWindow              ));
    assert!(offset_of!(ConsoleScreenBufferInfoEx, maximum_window_size   ) == offset_of!(CONSOLE_SCREEN_BUFFER_INFOEX, dwMaximumWindowSize   ));
    assert!(offset_of!(ConsoleScreenBufferInfoEx, popup_attributes      ) == offset_of!(CONSOLE_SCREEN_BUFFER_INFOEX, wPopupAttributes      ));
    assert!(offset_of!(ConsoleScreenBufferInfoEx, fullscreen_supported  ) == offset_of!(CONSOLE_SCREEN_BUFFER_INFOEX, bFullscreenSupported  ));
    assert!(offset_of!(ConsoleScreenBufferInfoEx, color_table           ) == offset_of!(CONSOLE_SCREEN_BUFFER_INFOEX, ColorTable            ));
};
