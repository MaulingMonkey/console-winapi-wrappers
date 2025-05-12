#[allow(unused_imports)] use crate::*;



#[doc(alias = "INPUT_RECORD::EventType")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/input-record-str)\]
/// [InputRecord]::[event_type](InputRecord::event_type)
///
#[derive(Clone, Copy, bytemuck::Pod, Default, bytemuck::Zeroable, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct InputRecordEventType(u16);

impl From<InputRecordEventType> for u16 { fn from(value: InputRecordEventType) -> Self { value.0 } }
impl From<u16> for InputRecordEventType { fn from(value: u16) -> Self { Self(value) } }

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/input-record-str)\]
/// [`InputRecord`] is a [`KeyEventRecord`]
///
pub const KEY_EVENT                 : InputRecordEventType = InputRecordEventType(0x0001);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/input-record-str)\]
/// [`InputRecord`] is a [`MouseEventRecord`]
///
pub const MOUSE_EVENT               : InputRecordEventType = InputRecordEventType(0x0002);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/input-record-str)\]
/// [`InputRecord`] is a [`WindowBufferSizeRecord`]
///
pub const WINDOW_BUFFER_SIZE_EVENT  : InputRecordEventType = InputRecordEventType(0x0004);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/input-record-str)\]
/// [`InputRecord`] is a [`MenuEventRecord`]
///
pub const MENU_EVENT                : InputRecordEventType = InputRecordEventType(0x0008);

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/input-record-str)\]
/// [`InputRecord`] is a [`FocusEventRecord`]
///
pub const FOCUS_EVENT               : InputRecordEventType = InputRecordEventType(0x0010);

const _ : () = {
    assert!(KEY_EVENT.0                 == winapi::um::wincon::KEY_EVENT                );
    assert!(MOUSE_EVENT.0               == winapi::um::wincon::MOUSE_EVENT              );
    assert!(WINDOW_BUFFER_SIZE_EVENT.0  == winapi::um::wincon::WINDOW_BUFFER_SIZE_EVENT );
    assert!(MENU_EVENT.0                == winapi::um::wincon::MENU_EVENT               );
    assert!(FOCUS_EVENT.0               == winapi::um::wincon::FOCUS_EVENT              );
};

impl core::fmt::Debug for InputRecordEventType {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        let s = match *self {
            KEY_EVENT                   => "KEY_EVENT",                 // 0x0001
            MOUSE_EVENT                 => "MOUSE_EVENT",               // 0x0002
            WINDOW_BUFFER_SIZE_EVENT    => "WINDOW_BUFFER_SIZE_EVENT",  // 0x0004
            MENU_EVENT                  => "MENU_EVENT",                // 0x0008
            FOCUS_EVENT                 => "FOCUS_EVENT",               // 0x0010
            Self(unknown)               => return write!(fmt, "???_EVENT (0x{unknown:04x})"),
        };
        write!(fmt, "{s}")
    }
}
