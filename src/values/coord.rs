use winapi::um::wincon::COORD;



#[doc(alias = "COORD")]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/coord-str)\]
/// struct { x: [i16], y: [i16] }
#[derive(Clone, Copy, bytemuck::Pod, Default, bytemuck::Zeroable, Debug, PartialEq, Eq)] // PartialOrd, Ord, Hash?
#[repr(C)] pub struct Coord {
    pub x: i16,
    pub y: i16,
}

impl From<Coord> for COORD { fn from(value: Coord) -> Self { unsafe { core::mem::transmute(value) } } }
impl From<COORD> for Coord { fn from(value: COORD) -> Self { unsafe { core::mem::transmute(value) } } }

const _ : () = {
    use core::mem::offset_of;
    assert!(align_of::<COORD>() == align_of::<Coord>());
    assert!(size_of ::<COORD>() == size_of ::<Coord>());
    assert!(offset_of!(Coord, x) == offset_of!(COORD, X));
    assert!(offset_of!(Coord, y) == offset_of!(COORD, Y));
};

#[test] fn layout() {
    let a = Coord::new(1, 2);
    let b : COORD = a.into();
    assert_eq!(a.x, b.X);
    assert_eq!(a.y, b.Y);
}

impl Coord {
    pub const fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

impl From<(i16, i16)> for Coord { fn from(value: (i16, i16) ) -> Self { let (x, y) = value; Self::new(x, y) } }
impl From<[i16; 2]  > for Coord { fn from(value: [i16; 2]   ) -> Self { let [x, y] = value; Self::new(x, y) } }
