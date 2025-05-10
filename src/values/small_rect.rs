use crate::Coord;
use winapi::um::wincon::{COORD, SMALL_RECT};
use core::ops::Range;



#[doc(alias = "SMALL_RECT")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/small-rect-str)\]
/// struct { left: [i16], top: [i16], right: [i16], bottom: [i16] }
#[derive(Clone, Copy, bytemuck::Pod, Default, bytemuck::Zeroable, Debug, PartialEq, Eq)] // PartialOrd, Ord, Hash?
#[repr(C)] pub struct SmallRect {
    /// The left side of the rectangle, inclusive.
    /// Generally, `left <= right`.
    pub left:   i16,

    /// The top side of the rectangle, inclusive.
    /// Generally, `top <= bottom`.
    pub top:    i16,

    /// The right side of the rectangle, exclusive.
    /// Generally, `left <= right`.
    pub right:  i16,

    /// The bottom side of the rectangle, exclusive.
    /// Generally, `top <= bottom`.
    pub bottom: i16,
}

impl From<SmallRect> for SMALL_RECT { fn from(value: SmallRect ) -> Self { unsafe { core::mem::transmute(value) } } }
impl From<SMALL_RECT> for SmallRect { fn from(value: SMALL_RECT) -> Self { unsafe { core::mem::transmute(value) } } }

const _ : () = {
    use core::mem::offset_of;
    assert!(align_of::<SMALL_RECT>() == align_of::<SmallRect>());
    assert!(size_of ::<SMALL_RECT>() == size_of ::<SmallRect>());
    assert!(offset_of!(SmallRect, left  ) == offset_of!(SMALL_RECT, Left  ));
    assert!(offset_of!(SmallRect, top   ) == offset_of!(SMALL_RECT, Top   ));
    assert!(offset_of!(SmallRect, right ) == offset_of!(SMALL_RECT, Right ));
    assert!(offset_of!(SmallRect, bottom) == offset_of!(SMALL_RECT, Bottom));
};

#[test] fn layout() {
    let a = SmallRect::new(1, 2, 3, 4);
    let b : SMALL_RECT = a.into();
    assert_eq!(a.left,   b.Left  );
    assert_eq!(a.top,    b.Top   );
    assert_eq!(a.right,  b.Right );
    assert_eq!(a.bottom, b.Bottom);
}

impl SmallRect {
    pub const fn new(left: i16, top: i16, right: i16, bottom: i16) -> Self {
        Self { left, top, right, bottom }
    }

    /// Returns the positive width of `self`, or [`None`] if the width was negative.
    ///
    /// ### Examples
    ///
    /// ```
    /// # use maulingmonkey_console_winapi_wrappers::*;
    /// assert_eq!(Some(10), SmallRect::new(-5, 0,  5, 0).width());
    /// assert_eq!(None,     SmallRect::new( 5, 0, -5, 0).width());
    /// assert_eq!(Some(0),  SmallRect::new( 0, 0,  0, 0).width());
    /// ```
    ///
    pub const fn width(&self) -> Option<u16> {
        let Self { left, right, .. } = *self;
        if left > right { return None }
        Some(right.wrapping_sub(left) as u16)
    }

    /// Returns the positive height of `self`, or [`None`] if the height was negative.
    ///
    /// ### Examples
    ///
    /// ```
    /// # use maulingmonkey_console_winapi_wrappers::*;
    /// assert_eq!(Some(10), SmallRect::new(0, -5, 0,  5).height());
    /// assert_eq!(None,     SmallRect::new(0,  5, 0, -5).height());
    /// assert_eq!(Some(0),  SmallRect::new(0,  0, 0,  0).height());
    /// ```
    ///
    pub const fn height(&self) -> Option<u16> {
        let Self { top, bottom, .. } = *self;
        if top > bottom { return None }
        Some(bottom.wrapping_sub(top) as u16)
    }

    /// Returns the positive width and height of `self`, or [`None`] if either dimension was negative.
    ///
    /// ### Examples
    ///
    /// ```
    /// # use maulingmonkey_console_winapi_wrappers::*;
    /// assert_eq!(Some((10, 10)),  SmallRect::new(-5, -5,  5,  5).size());
    /// assert_eq!(None,            SmallRect::new(-5,  5,  5, -5).size()); // negative height
    /// assert_eq!(None,            SmallRect::new( 5, -5, -5,  5).size()); // negative width
    /// assert_eq!(None,            SmallRect::new( 5,  5, -5, -5).size()); // negative both
    /// assert_eq!(Some((0, 0)),    SmallRect::new( 0,  0,  0,  0).size());
    /// ```
    ///
    pub const fn size(&self) -> Option<(u16, u16)> {
        let Some(w) = self.width()  else { return None };
        let Some(h) = self.height() else { return None };
        Some((w, h))
    }

    /// Returns the total area of `self`, or [`None`] if either the width or height was negative.
    ///
    /// ### Examples
    ///
    /// ```
    /// # use maulingmonkey_console_winapi_wrappers::*;
    /// assert_eq!(Some(100),   SmallRect::new(-5, -5,  5,  5).area());
    /// assert_eq!(None,        SmallRect::new(-5,  5,  5, -5).area()); // negative height
    /// assert_eq!(None,        SmallRect::new( 5, -5, -5,  5).area()); // negative width
    /// assert_eq!(None,        SmallRect::new( 5,  5, -5, -5).area()); // negative both
    /// assert_eq!(Some(0),     SmallRect::new( 0,  0,  0,  0).area());
    /// ```
    ///
    pub const fn area(&self) -> Option<u32> {
        let Some((w, h)) = self.size() else { return None };
        Some((w as u32).wrapping_mul(h as u32))
    }
}

impl From<Range<Coord>              > for SmallRect { fn from(value: Range<Coord>               ) -> Self { Self { left: value.start.x, top: value.start.y, right: value.end.x, bottom: value.end.y } } }
impl From<Range<COORD>              > for SmallRect { fn from(value: Range<COORD>               ) -> Self { Self { left: value.start.X, top: value.start.Y, right: value.end.X, bottom: value.end.Y } } }
impl From<Range<(i16, i16)>         > for SmallRect { fn from(value: Range<(i16, i16)>          ) -> Self { Self { left: value.start.0, top: value.start.1, right: value.end.0, bottom: value.end.1 } } }
impl From<(Range<i16>, Range<i16>)  > for SmallRect { fn from(value: (Range<i16>, Range<i16>)   ) -> Self { Self { left: value.0.start, top: value.1.start, right: value.0.end, bottom: value.1.end } } }
impl From<Range<[i16; 2]>           > for SmallRect { fn from(value: Range<[i16; 2]>            ) -> Self { let [left, right] = value.start; let [top, bottom] = value.end; Self { left, top, right, bottom } } }
impl From<[Range<i16>; 2]           > for SmallRect { fn from(value: [Range<i16>; 2]            ) -> Self { let [x, y] = value; Self { left: x.start, top: y.start, right: x.end, bottom: y.end } } }
