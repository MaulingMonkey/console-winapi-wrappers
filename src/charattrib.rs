use winapi::shared::minwindef::WORD;
use winapi::um::wincon;

use std::ops::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/console-screen-buffers#character-attributes)\]
/// Character attributes can be divided into two classes: color and DBCS. The following attributes are defined in the WinCon.h header file.
#[repr(transparent)] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)] pub struct Attributes(WORD);



/// Text color contains blue.
#[doc(hidden)] pub const FOREGROUND_BLUE : Attributes = Attributes(wincon::FOREGROUND_BLUE);

/// Text color contains green.
#[doc(hidden)] pub const FOREGROUND_GREEN : Attributes = Attributes(wincon::FOREGROUND_GREEN);

/// Text color contains red.
#[doc(hidden)] pub const FOREGROUND_RED : Attributes = Attributes(wincon::FOREGROUND_RED);

/// Text color is intensified.
#[doc(hidden)] pub const FOREGROUND_INTENSITY : Attributes = Attributes(wincon::FOREGROUND_INTENSITY);

/// Background color contains blue.
#[doc(hidden)] pub const BACKGROUND_BLUE : Attributes = Attributes(wincon::BACKGROUND_BLUE);

/// Background color contains green.
#[doc(hidden)] pub const BACKGROUND_GREEN : Attributes = Attributes(wincon::BACKGROUND_GREEN);

/// Background color contains red.
#[doc(hidden)] pub const BACKGROUND_RED : Attributes = Attributes(wincon::BACKGROUND_RED);

/// Background color is intensified.
#[doc(hidden)] pub const BACKGROUND_INTENSITY : Attributes = Attributes(wincon::BACKGROUND_INTENSITY);

/// Leading byte.
#[doc(hidden)] pub const COMMON_LVB_LEADING_BYTE : Attributes = Attributes(wincon::COMMON_LVB_LEADING_BYTE);

/// Trailing byte.
#[doc(hidden)] pub const COMMON_LVB_TRAILING_BYTE : Attributes = Attributes(wincon::COMMON_LVB_TRAILING_BYTE);

/// Top horizontal.
#[doc(hidden)] pub const COMMON_LVB_GRID_HORIZONTAL : Attributes = Attributes(wincon::COMMON_LVB_GRID_HORIZONTAL);

/// Left vertical.
#[doc(hidden)] pub const COMMON_LVB_GRID_LVERTICAL : Attributes = Attributes(wincon::COMMON_LVB_GRID_LVERTICAL);

/// Right vertical.
#[doc(hidden)] pub const COMMON_LVB_GRID_RVERTICAL : Attributes = Attributes(wincon::COMMON_LVB_GRID_RVERTICAL);

/// Reverse foreground and background attributes.
#[doc(hidden)] pub const COMMON_LVB_REVERSE_VIDEO : Attributes = Attributes(wincon::COMMON_LVB_REVERSE_VIDEO);

/// Underscore.
#[doc(hidden)] pub const COMMON_LVB_UNDERSCORE : Attributes = Attributes(wincon::COMMON_LVB_UNDERSCORE);

impl Attributes {
    /// Text color contains blue.
    pub const FOREGROUND_BLUE : Attributes = Attributes(wincon::FOREGROUND_BLUE);

    /// Text color contains green.
    pub const FOREGROUND_GREEN : Attributes = Attributes(wincon::FOREGROUND_GREEN);

    /// Text color contains red.
    pub const FOREGROUND_RED : Attributes = Attributes(wincon::FOREGROUND_RED);

    /// Text color is intensified.
    pub const FOREGROUND_INTENSITY : Attributes = Attributes(wincon::FOREGROUND_INTENSITY);

    /// Background color contains blue.
    pub const BACKGROUND_BLUE : Attributes = Attributes(wincon::BACKGROUND_BLUE);

    /// Background color contains green.
    pub const BACKGROUND_GREEN : Attributes = Attributes(wincon::BACKGROUND_GREEN);

    /// Background color contains red.
    pub const BACKGROUND_RED : Attributes = Attributes(wincon::BACKGROUND_RED);

    /// Background color is intensified.
    pub const BACKGROUND_INTENSITY : Attributes = Attributes(wincon::BACKGROUND_INTENSITY);

    /// Leading byte.
    pub const COMMON_LVB_LEADING_BYTE : Attributes = Attributes(wincon::COMMON_LVB_LEADING_BYTE);

    /// Trailing byte.
    pub const COMMON_LVB_TRAILING_BYTE : Attributes = Attributes(wincon::COMMON_LVB_TRAILING_BYTE);

    /// Top horizontal.
    pub const COMMON_LVB_GRID_HORIZONTAL : Attributes = Attributes(wincon::COMMON_LVB_GRID_HORIZONTAL);

    /// Left vertical.
    pub const COMMON_LVB_GRID_LVERTICAL : Attributes = Attributes(wincon::COMMON_LVB_GRID_LVERTICAL);

    /// Right vertical.
    pub const COMMON_LVB_GRID_RVERTICAL : Attributes = Attributes(wincon::COMMON_LVB_GRID_RVERTICAL);

    /// Reverse foreground and background attributes.
    pub const COMMON_LVB_REVERSE_VIDEO : Attributes = Attributes(wincon::COMMON_LVB_REVERSE_VIDEO);

    /// Underscore.
    pub const COMMON_LVB_UNDERSCORE : Attributes = Attributes(wincon::COMMON_LVB_UNDERSCORE);
}

impl From<WORD> for Attributes { fn from(value: WORD) -> Self { Self(value) } }
impl From<Attributes> for WORD { fn from(value: Attributes) -> Self { value.0 } }

impl BitAnd for Attributes { type Output = Self; fn bitand(self, rhs: Self) -> Self::Output { Self(self.0 & rhs.0) } }
impl BitXor for Attributes { type Output = Self; fn bitxor(self, rhs: Self) -> Self::Output { Self(self.0 ^ rhs.0) } }
impl BitOr  for Attributes { type Output = Self; fn bitor (self, rhs: Self) -> Self::Output { Self(self.0 | rhs.0) } }
impl Not    for Attributes { type Output = Self; fn not   (self)            -> Self::Output { Self(!self.0) } }

impl BitAndAssign for Attributes { fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; } }
impl BitXorAssign for Attributes { fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; } }
impl BitOrAssign  for Attributes { fn bitor_assign (&mut self, rhs: Self) { self.0 |= rhs.0; } }
