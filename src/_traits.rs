use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::SHORT;
use winapi::um::wincontypes::{COORD, SMALL_RECT};

use std::ops::*;



/// `DWORD` / <code>&[std::process::Child]</code>
pub trait IntoProcessId {
    fn into_process_id(self) -> DWORD;
}

impl IntoProcessId for DWORD                    { fn into_process_id(self) -> DWORD { self } }
impl IntoProcessId for &'_ std::process::Child  { fn into_process_id(self) -> DWORD { self.id() } }



/// `COORD` / `(SHORT, SHORT)` / `[SHORT; 2]`
pub trait IntoCoord {
    fn into_coord(self) -> COORD;
}

impl IntoCoord for COORD { fn into_coord(self) -> COORD { self } }
impl IntoCoord for (SHORT, SHORT) { fn into_coord(self) -> COORD { #[allow(non_snake_case)] let (X, Y) = self; COORD { X, Y } } }
impl IntoCoord for [SHORT; 2] { fn into_coord(self) -> COORD { #[allow(non_snake_case)] let [X, Y] = self; COORD { X, Y } } }



/// `SMALL_RECT` / `COORD { X: 1, Y: 2 } .. COORD { X: 3, Y: 4 }` / `(1,2)..(3,4)` / `(1..3, 2..4)` / `[1,2]..[3,4]` / `[1..3, 2..4]`
pub trait IntoSmallRect {
    fn into_small_rect(self) -> SMALL_RECT;
}

impl IntoSmallRect for SMALL_RECT                   { fn into_small_rect(self) -> SMALL_RECT { self } }
impl IntoSmallRect for Range<COORD>                 { fn into_small_rect(self) -> SMALL_RECT { SMALL_RECT { Left: self.start.X, Top: self.start.Y, Right: self.end.X, Bottom: self.end.Y } } }
impl IntoSmallRect for Range<(SHORT, SHORT)>        { fn into_small_rect(self) -> SMALL_RECT { SMALL_RECT { Left: self.start.0, Top: self.start.1, Right: self.end.0, Bottom: self.end.1 } } }
impl IntoSmallRect for (Range<SHORT>, Range<SHORT>) { fn into_small_rect(self) -> SMALL_RECT { SMALL_RECT { Left: self.0.start, Top: self.1.start, Right: self.0.end, Bottom: self.1.end } } }
impl IntoSmallRect for Range<[SHORT; 2]>            { fn into_small_rect(self) -> SMALL_RECT { SMALL_RECT { Left: self.start[0], Top: self.start[1], Right: self.end[0], Bottom: self.end[1] } } }
impl IntoSmallRect for [Range<SHORT>; 2]            { fn into_small_rect(self) -> SMALL_RECT { SMALL_RECT { Left: self[0].start, Top: self[1].start, Right: self[0].end, Bottom: self[1].end } } }



/// [`()`](https://doc.rust-lang.org/std/primitive.unit.html)
pub trait Reserved : sealed::Reserved {}
impl Reserved for () {}

mod sealed {
    pub trait Reserved {}
    impl Reserved for () {}
}
