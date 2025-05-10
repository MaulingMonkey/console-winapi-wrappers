#[doc(alias = "COLORREF")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/gdi/colorref)\]
/// 0x00RRGGBB color.
///
#[derive(Clone, Copy, bytemuck::Pod, Default, bytemuck::Zeroable, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ColorRef(u32);

impl From<u32> for ColorRef { fn from(value: u32) -> Self { Self(value) } }
impl From<ColorRef> for u32 { fn from(value: ColorRef) -> Self { value.0 } }

impl ColorRef {
    #[doc(alias = "RGB")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-rgb)\]
    /// RGB
    /// &mdash; construct a color from red, green, and blue components (0 ..= 255).
    ///
    pub const fn from_rgb([red, green, blue]: [u8; 3]) -> Self {
        Self(
            ((red   as u32) << 16) |
            ((green as u32) <<  8) |
            ((blue  as u32) <<  0)
        )
    }

    /// \[~~microsoft.com~~\]
    /// Decompose `self` into red, green, and blue components (0 ..= 255).
    pub const fn to_rgb(self) -> [u8; 3] {
        [self.red(), self.green(), self.blue()]
    }

    #[doc(alias = "GetRValue")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getrvalue)\]
    /// GetRValue
    /// &mdash; get the red component (0 ..= 255) of the color.
    ///
    pub const fn red(self) -> u8 { (self.0 >> 16) as u8 }

    #[doc(alias = "GetGValue")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getgvalue)\]
    /// GetGValue
    /// &mdash; get the green component (0 ..= 255) of the color.
    ///
    pub const fn green(self) -> u8 { (self.0 >>  8) as u8 }

    #[doc(alias = "GetBValue")]
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getbvalue)\]
    /// GetBValue
    /// &mdash; get the blue component (0 ..= 255) of the color.
    ///
    pub const fn blue(self) -> u8 { (self.0 >>  0) as u8 }
}
