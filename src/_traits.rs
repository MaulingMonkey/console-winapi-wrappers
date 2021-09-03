use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::SHORT;
use winapi::um::wincontypes::COORD;



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
