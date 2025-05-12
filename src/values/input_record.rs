use crate::{AsciiOrUnicodeChar, Coord, InputRecordEventType};
use crate::{FOCUS_EVENT, KEY_EVENT, MENU_EVENT, MOUSE_EVENT, WINDOW_BUFFER_SIZE_EVENT};
use winapi::um::wincon::{FOCUS_EVENT_RECORD, INPUT_RECORD, KEY_EVENT_RECORD, MENU_EVENT_RECORD, MOUSE_EVENT_RECORD, WINDOW_BUFFER_SIZE_RECORD};
use bytemuck::Zeroable;
use core::convert::{TryFrom, TryInto as _};



#[doc(alias = "INPUT_RECORD")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/input-record-str)\]
/// [KeyEventRecord] | [MouseEventRecord] | [WindowBufferSizeRecord] | ...
///
#[derive(Clone, Copy, Default)]
#[repr(transparent)] pub struct InputRecord(INPUT_RECORD);

//safe impl bytemuck::Pod      for InputRecord {} // XXX: `InputRecord` can probably have uninit/padding bytes, depending on active union member
unsafe impl bytemuck::Zeroable for InputRecord {}

impl InputRecord {
    /// ### Safety
    /// `input_record.Event.*` must be properly initialized to match `input_record.EventType`.
    /// As all known fields are Pod, this is mostly a matter of avoiding uninitialized padding bytes.
    ///
    /// ### Alternatives
    ///
    /// <code>impl [From]&lt;...&gt; for [InputRecord]</code> is a safe alternative, implemented for all known record types:
    ///
    /// ```
    /// # use maulingmonkey_console_winapi_wrappers::*;
    /// let input_record : InputRecord = KeyEventRecord::default().into();
    /// let input_record : InputRecord = MouseEventRecord::default().into();
    /// let input_record : InputRecord = WindowBufferSizeRecord::default().into();
    /// let input_record : InputRecord = MenuEventRecord::default().into();
    /// let input_record : InputRecord = FocusEventRecord::default().into();
    /// ```
    ///
    pub const unsafe fn from_unchecked(input_record: INPUT_RECORD) -> Self { Self(input_record) }

    /// Returns one of [`KEY_EVENT`], [`MOUSE_EVENT`], [`WINDOW_BUFFER_SIZE_EVENT`], ...
    pub fn event_type(&self) -> InputRecordEventType { InputRecordEventType::from(self.0.EventType) }

    /// Returns <code>[Some]\(...\)</code> if <code>[event_type](Self::event_type)()</code> is [`KEY_EVENT`].
    pub fn as_key_event(&self)                  -> Option<&KeyEventRecord           > { self.try_into().ok() }

    /// Returns <code>[Some]\(...\)</code> if <code>[event_type](Self::event_type)()</code> is [`MOUSE_EVENT`].
    pub fn as_mouse_event(&self)                -> Option<&MouseEventRecord         > { self.try_into().ok() }

    /// Returns <code>[Some]\(...\)</code> if <code>[event_type](Self::event_type)()</code> is [`WINDOW_BUFFER_SIZE_EVENT`].
    pub fn as_window_buffer_size_event(&self)   -> Option<&WindowBufferSizeRecord   > { self.try_into().ok() }

    /// Returns <code>[Some]\(...\)</code> if <code>[event_type](Self::event_type)()</code> is [`MENU_EVENT`].
    pub fn as_menu_event(&self)                 -> Option<&MenuEventRecord          > { self.try_into().ok() }

    /// Returns <code>[Some]\(...\)</code> if <code>[event_type](Self::event_type)()</code> is [`FOCUS_EVENT`].
    pub fn as_focus_event(&self)                -> Option<&FocusEventRecord         > { self.try_into().ok() }
}

impl AsRef<INPUT_RECORD> for InputRecord { fn as_ref(&self) -> &INPUT_RECORD { &self.0 } }
//pl AsMut<INPUT_RECORD> for InputRecord { fn as_mut(&self) -> &INPUT_RECORD { &self.0 } } // XXX: do not implement: `Event` might not be fully initialized for a different `EventType`
impl From<InputRecord> for INPUT_RECORD { fn from(value: InputRecord) -> Self { value.0 } }

impl core::fmt::Debug for InputRecord {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        if      let Some(e) = self.as_key_event()                   { fmt.debug_tuple("InputRecord").field(e).finish() }
        else if let Some(e) = self.as_mouse_event()                 { fmt.debug_tuple("InputRecord").field(e).finish() }
        else if let Some(e) = self.as_window_buffer_size_event()    { fmt.debug_tuple("InputRecord").field(e).finish() }
        else if let Some(e) = self.as_menu_event()                  { fmt.debug_tuple("InputRecord").field(e).finish() }
        else if let Some(e) = self.as_focus_event()                 { fmt.debug_tuple("InputRecord").field(e).finish() }
        else {
            fmt.debug_struct("InputRecord")
                .field("event_type", &self.event_type())
                .finish_non_exhaustive()
        }
    }
}



#[doc(alias = "KEY_EVENT_RECORD")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/key-event-record-str)\]
/// [InputRecord] indicating keyboard input
///
#[derive(Clone, Copy, bytemuck::Pod, Debug, Default, bytemuck::Zeroable)]
#[repr(C)] pub struct KeyEventRecord {
    pub key_down:           abibool::bool32,
    pub repeat_count:       u16,
    pub virtual_key_code:   u16, // XXX: better types?
    pub virtual_scan_code:  u16,
    pub char:               AsciiOrUnicodeChar,
    pub control_key_state:  u32, // XXX: better types?
}

impl AsRef<KeyEventRecord> for KEY_EVENT_RECORD { fn as_ref(&self) -> &KeyEventRecord   { unsafe { core::mem::transmute(self) } } }
impl AsRef<KEY_EVENT_RECORD> for KeyEventRecord { fn as_ref(&self) -> &KEY_EVENT_RECORD { unsafe { core::mem::transmute(self) } } }
impl From<KeyEventRecord> for KEY_EVENT_RECORD { fn from(value: KeyEventRecord  ) -> Self { unsafe { core::mem::transmute(value) } } }
impl From<KEY_EVENT_RECORD> for KeyEventRecord { fn from(value: KEY_EVENT_RECORD) -> Self { unsafe { core::mem::transmute(value) } } }

const _ : () = {
    use core::mem::offset_of;
    assert!(align_of::<KEY_EVENT_RECORD>() == align_of::<KeyEventRecord>());
    assert!(size_of ::<KEY_EVENT_RECORD>() == size_of ::<KeyEventRecord>());
    assert!(offset_of!(KeyEventRecord, key_down         ) == offset_of!(KEY_EVENT_RECORD, bKeyDown          ));
    assert!(offset_of!(KeyEventRecord, repeat_count     ) == offset_of!(KEY_EVENT_RECORD, wRepeatCount      ));
    assert!(offset_of!(KeyEventRecord, virtual_key_code ) == offset_of!(KEY_EVENT_RECORD, wVirtualKeyCode   ));
    assert!(offset_of!(KeyEventRecord, virtual_scan_code) == offset_of!(KEY_EVENT_RECORD, wVirtualScanCode  ));
    assert!(offset_of!(KeyEventRecord, char             ) == offset_of!(KEY_EVENT_RECORD, uChar             ));
    assert!(offset_of!(KeyEventRecord, control_key_state) == offset_of!(KEY_EVENT_RECORD, dwControlKeyState ));
};

impl From<KeyEventRecord> for InputRecord {
    fn from(value: KeyEventRecord) -> Self {
        let mut r = InputRecord::zeroed();
        r.0.EventType = KEY_EVENT.into();
        unsafe { *r.0.Event.KeyEvent_mut() = value.into() };
        r
    }
}

impl<'a> TryFrom<&'a InputRecord> for &'a KeyEventRecord {
    type Error = ();
    fn try_from(value: &'a InputRecord) -> Result<Self, Self::Error> {
        if value.event_type() == KEY_EVENT {
            Ok(unsafe { value.0.Event.KeyEvent() }.as_ref())
        } else {
            Err(())
        }
    }
}



#[doc(alias = "MENU_EVENT_RECORD")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/menu-event-record-str)\]
/// Internal [InputRecord] Microsoft recommends ignoring.
///
#[derive(Clone, Copy, bytemuck::Pod, Debug, Default, bytemuck::Zeroable)]
#[repr(C)] pub struct MenuEventRecord {
    pub command_id:     u32,
}

impl AsRef<MenuEventRecord> for MENU_EVENT_RECORD { fn as_ref(&self) -> &MenuEventRecord   { unsafe { core::mem::transmute(self) } } }
impl AsRef<MENU_EVENT_RECORD> for MenuEventRecord { fn as_ref(&self) -> &MENU_EVENT_RECORD { unsafe { core::mem::transmute(self) } } }
impl From<MenuEventRecord> for MENU_EVENT_RECORD { fn from(value: MenuEventRecord  ) -> Self { unsafe { core::mem::transmute(value) } } }
impl From<MENU_EVENT_RECORD> for MenuEventRecord { fn from(value: MENU_EVENT_RECORD) -> Self { unsafe { core::mem::transmute(value) } } }

const _ : () = {
    use core::mem::offset_of;
    assert!(align_of::<MENU_EVENT_RECORD>() == align_of::<MenuEventRecord>());
    assert!(size_of ::<MENU_EVENT_RECORD>() == size_of ::<MenuEventRecord>());
    assert!(offset_of!(MenuEventRecord, command_id) == offset_of!(MENU_EVENT_RECORD, dwCommandId));
};

impl From<MenuEventRecord> for InputRecord {
    fn from(value: MenuEventRecord) -> Self {
        let mut r = InputRecord::zeroed();
        r.0.EventType = MENU_EVENT.into();
        unsafe { *r.0.Event.MenuEvent_mut() = value.into() };
        r
    }
}

impl<'a> TryFrom<&'a InputRecord> for &'a MenuEventRecord {
    type Error = ();
    fn try_from(value: &'a InputRecord) -> Result<Self, Self::Error> {
        if value.event_type() == MENU_EVENT {
            Ok(unsafe { value.0.Event.MenuEvent() }.as_ref())
        } else {
            Err(())
        }
    }
}



#[doc(alias = "MOUSE_EVENT_RECORD")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/mouse-event-record-str)\]
/// [InputRecord] indicating mouse movement or interaction.
///
#[derive(Clone, Copy, bytemuck::Pod, Debug, Default, bytemuck::Zeroable)]
#[repr(C)] pub struct MouseEventRecord {
    pub mouse_position:     Coord,
    pub button_state:       u32, // XXX: better type?
    pub control_key_state:  u32, // XXX: better type?
    pub event_flags:        u32, // XXX: better type?
}

impl AsRef<MouseEventRecord> for MOUSE_EVENT_RECORD { fn as_ref(&self) -> &MouseEventRecord   { unsafe { core::mem::transmute(self) } } }
impl AsRef<MOUSE_EVENT_RECORD> for MouseEventRecord { fn as_ref(&self) -> &MOUSE_EVENT_RECORD { unsafe { core::mem::transmute(self) } } }
impl From<MouseEventRecord> for MOUSE_EVENT_RECORD { fn from(value: MouseEventRecord  ) -> Self { unsafe { core::mem::transmute(value) } } }
impl From<MOUSE_EVENT_RECORD> for MouseEventRecord { fn from(value: MOUSE_EVENT_RECORD) -> Self { unsafe { core::mem::transmute(value) } } }

const _ : () = {
    use core::mem::offset_of;
    assert!(align_of::<MOUSE_EVENT_RECORD>() == align_of::<MouseEventRecord>());
    assert!(size_of ::<MOUSE_EVENT_RECORD>() == size_of ::<MouseEventRecord>());
    assert!(offset_of!(MouseEventRecord, mouse_position     ) == offset_of!(MOUSE_EVENT_RECORD, dwMousePosition     ));
    assert!(offset_of!(MouseEventRecord, button_state       ) == offset_of!(MOUSE_EVENT_RECORD, dwButtonState       ));
    assert!(offset_of!(MouseEventRecord, control_key_state  ) == offset_of!(MOUSE_EVENT_RECORD, dwControlKeyState   ));
    assert!(offset_of!(MouseEventRecord, event_flags        ) == offset_of!(MOUSE_EVENT_RECORD, dwEventFlags        ));
};

impl From<MouseEventRecord> for InputRecord {
    fn from(value: MouseEventRecord) -> Self {
        let mut r = InputRecord::zeroed();
        r.0.EventType = MOUSE_EVENT.into();
        unsafe { *r.0.Event.MouseEvent_mut() = value.into() };
        r
    }
}

impl<'a> TryFrom<&'a InputRecord> for &'a MouseEventRecord {
    type Error = ();
    fn try_from(value: &'a InputRecord) -> Result<Self, Self::Error> {
        if value.event_type() == MOUSE_EVENT {
            Ok(unsafe { value.0.Event.MouseEvent() }.as_ref())
        } else {
            Err(())
        }
    }
}



#[doc(alias = "FOCUS_EVENT_RECORD")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/focus-event-record-str)\]
/// Internal [InputRecord] Microsoft recommends ignoring.
///
#[derive(Clone, Copy, bytemuck::Pod, Debug, Default, bytemuck::Zeroable)]
#[repr(C)] pub struct FocusEventRecord {
    pub set_focus:  abibool::bool32,
}

impl AsRef<FocusEventRecord> for FOCUS_EVENT_RECORD { fn as_ref(&self) -> &FocusEventRecord   { unsafe { core::mem::transmute(self) } } }
impl AsRef<FOCUS_EVENT_RECORD> for FocusEventRecord { fn as_ref(&self) -> &FOCUS_EVENT_RECORD { unsafe { core::mem::transmute(self) } } }
impl From<FocusEventRecord> for FOCUS_EVENT_RECORD { fn from(value: FocusEventRecord  ) -> Self { unsafe { core::mem::transmute(value) } } }
impl From<FOCUS_EVENT_RECORD> for FocusEventRecord { fn from(value: FOCUS_EVENT_RECORD) -> Self { unsafe { core::mem::transmute(value) } } }

const _ : () = {
    use core::mem::offset_of;
    assert!(align_of::<FOCUS_EVENT_RECORD>() == align_of::<FocusEventRecord>());
    assert!(size_of ::<FOCUS_EVENT_RECORD>() == size_of ::<FocusEventRecord>());
    assert!(offset_of!(FocusEventRecord, set_focus) == offset_of!(FOCUS_EVENT_RECORD, bSetFocus));
};

impl From<FocusEventRecord> for InputRecord {
    fn from(value: FocusEventRecord) -> Self {
        let mut r = InputRecord::zeroed();
        r.0.EventType = FOCUS_EVENT.into();
        unsafe { *r.0.Event.FocusEvent_mut() = value.into() };
        r
    }
}

impl<'a> TryFrom<&'a InputRecord> for &'a FocusEventRecord {
    type Error = ();
    fn try_from(value: &'a InputRecord) -> Result<Self, Self::Error> {
        if value.event_type() == FOCUS_EVENT {
            Ok(unsafe { value.0.Event.FocusEvent() }.as_ref())
        } else {
            Err(())
        }
    }
}



#[doc(alias = "WINDOW_BUFFER_SIZE_RECORD")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/focus-event-record-str)\]
/// [InputRecord] indicating the console screen buffer was resized
///
#[derive(Clone, Copy, bytemuck::Pod, Debug, Default, bytemuck::Zeroable)]
#[repr(C)] pub struct WindowBufferSizeRecord {
    /// New size of the console screen buffer, in cells.
    pub size: Coord,
}

impl AsRef<WindowBufferSizeRecord> for WINDOW_BUFFER_SIZE_RECORD { fn as_ref(&self) -> &WindowBufferSizeRecord    { unsafe { core::mem::transmute(self) } } }
impl AsRef<WINDOW_BUFFER_SIZE_RECORD> for WindowBufferSizeRecord { fn as_ref(&self) -> &WINDOW_BUFFER_SIZE_RECORD { unsafe { core::mem::transmute(self) } } }
impl From<WindowBufferSizeRecord> for WINDOW_BUFFER_SIZE_RECORD  { fn from(value: WindowBufferSizeRecord   ) -> Self { unsafe { core::mem::transmute(value) } } }
impl From<WINDOW_BUFFER_SIZE_RECORD> for WindowBufferSizeRecord  { fn from(value: WINDOW_BUFFER_SIZE_RECORD) -> Self { unsafe { core::mem::transmute(value) } } }

const _ : () = {
    use core::mem::offset_of;
    assert!(align_of::<WINDOW_BUFFER_SIZE_RECORD>() == align_of::<WindowBufferSizeRecord>());
    assert!(size_of ::<WINDOW_BUFFER_SIZE_RECORD>() == size_of ::<WindowBufferSizeRecord>());
    assert!(offset_of!(WindowBufferSizeRecord, size) == offset_of!(WINDOW_BUFFER_SIZE_RECORD, dwSize));
};

impl From<WindowBufferSizeRecord> for InputRecord {
    fn from(value: WindowBufferSizeRecord) -> Self {
        let mut r = InputRecord::zeroed();
        r.0.EventType = WINDOW_BUFFER_SIZE_EVENT.into();
        unsafe { *r.0.Event.WindowBufferSizeEvent_mut() = value.into() };
        r
    }
}

impl<'a> TryFrom<&'a InputRecord> for &'a WindowBufferSizeRecord {
    type Error = ();
    fn try_from(value: &'a InputRecord) -> Result<Self, Self::Error> {
        if value.event_type() == WINDOW_BUFFER_SIZE_EVENT {
            Ok(unsafe { value.0.Event.WindowBufferSizeEvent() }.as_ref())
        } else {
            Err(())
        }
    }
}
