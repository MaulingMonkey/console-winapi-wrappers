use winapi::um::wincon::CONSOLE_CURSOR_INFO;



#[doc(alias = "CONSOLE_CURSOR_INFO")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/console-cursor-info-str)\]
/// Cursor visibility and cell coverage percentage (1 ..= 100).
///
#[derive(Clone, Copy, bytemuck::Pod, Debug, Default, bytemuck::Zeroable)]
#[repr(C)] pub struct ConsoleCursorInfo {
    /// The percentage of the character cell (`1 ..= 100`) filled by the cursor, starting from a horizontal line at the bottom.
    pub size:       u32,

    /// If the cursor is visible at all.
    pub visible:    abibool::bool32,
}

impl ConsoleCursorInfo {
    pub const fn new(size: u32, visible: bool) -> Self {
        Self { size, visible: if visible { abibool::bool32::TRUE } else { abibool::bool32::FALSE } }
    }
}

impl From<ConsoleCursorInfo> for CONSOLE_CURSOR_INFO { fn from(value: ConsoleCursorInfo  ) -> Self { unsafe { core::mem::transmute(value) } } }
impl From<CONSOLE_CURSOR_INFO> for ConsoleCursorInfo { fn from(value: CONSOLE_CURSOR_INFO) -> Self { unsafe { core::mem::transmute(value) } } }

const _ : () = {
    use core::mem::offset_of;
    assert!(align_of::<CONSOLE_CURSOR_INFO>() == align_of::<ConsoleCursorInfo>());
    assert!(size_of ::<CONSOLE_CURSOR_INFO>() == size_of ::<ConsoleCursorInfo>());
    assert!(offset_of!(ConsoleCursorInfo, size   ) == offset_of!(CONSOLE_CURSOR_INFO, dwSize    ));
    assert!(offset_of!(ConsoleCursorInfo, visible) == offset_of!(CONSOLE_CURSOR_INFO, bVisible  ));
};

#[test] fn layout() {
    let a = ConsoleCursorInfo::new(100, true);
    let b : CONSOLE_CURSOR_INFO = a.into();
    assert_eq!(a.size,      b.dwSize        );
    assert_eq!(a.visible,   b.bVisible != 0 );
}
