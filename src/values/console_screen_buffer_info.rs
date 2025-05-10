use crate::{Attributes, Coord, SmallRect};
use winapi::um::wincon::CONSOLE_SCREEN_BUFFER_INFO;



#[doc(alias = "CONSOLE_SCREEN_BUFFER_INFO")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/console-screen-buffer-info-str)\]
///
#[derive(Clone, Copy, bytemuck::Pod, Default, bytemuck::Zeroable, Debug)] // PartialEq, Eq, PartialOrd, Ord, Hash?
#[repr(C)] pub struct ConsoleScreenBufferInfo {
    pub size:                   Coord,
    pub cursor_position:        Coord,
    pub attributes:             Attributes,
    pub window:                 SmallRect,
    pub maximum_window_size:    Coord,
}

impl From<CONSOLE_SCREEN_BUFFER_INFO> for ConsoleScreenBufferInfo { fn from(value: CONSOLE_SCREEN_BUFFER_INFO   ) -> Self { unsafe { core::mem::transmute(value) } } }
impl From<ConsoleScreenBufferInfo> for CONSOLE_SCREEN_BUFFER_INFO { fn from(value: ConsoleScreenBufferInfo      ) -> Self { unsafe { core::mem::transmute(value) } } }

const _ : () = {
    use core::mem::offset_of;
    assert!(align_of::<CONSOLE_SCREEN_BUFFER_INFO>() == align_of::<ConsoleScreenBufferInfo>());
    assert!(size_of ::<CONSOLE_SCREEN_BUFFER_INFO>() == size_of ::<ConsoleScreenBufferInfo>());
    assert!(offset_of!(ConsoleScreenBufferInfo, size                ) == offset_of!(CONSOLE_SCREEN_BUFFER_INFO, dwSize              ));
    assert!(offset_of!(ConsoleScreenBufferInfo, cursor_position     ) == offset_of!(CONSOLE_SCREEN_BUFFER_INFO, dwCursorPosition    ));
    assert!(offset_of!(ConsoleScreenBufferInfo, attributes          ) == offset_of!(CONSOLE_SCREEN_BUFFER_INFO, wAttributes         ));
    assert!(offset_of!(ConsoleScreenBufferInfo, window              ) == offset_of!(CONSOLE_SCREEN_BUFFER_INFO, srWindow            ));
    assert!(offset_of!(ConsoleScreenBufferInfo, maximum_window_size ) == offset_of!(CONSOLE_SCREEN_BUFFER_INFO, dwMaximumWindowSize ));
};
