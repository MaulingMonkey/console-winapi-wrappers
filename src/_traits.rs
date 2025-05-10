use winapi::shared::minwindef::DWORD;



/// Convert into a [`DWORD`] process ID.
/// Implemented for:
/// [`DWORD`] | [`&std::process::Child`]
///
/// [`DWORD`]:                  https://docs.rs/winapi/0.3.9/winapi/shared/ntdef/type.DWORD.html
/// [`&std::process::Child`]:   https://doc.rust-lang.org/std/process/struct.Child.html
pub trait IntoProcessId {
    fn into_process_id(self) -> DWORD;
}

impl IntoProcessId for DWORD                    { fn into_process_id(self) -> DWORD { self } }
impl IntoProcessId for &'_ std::process::Child  { fn into_process_id(self) -> DWORD { self.id() } }



/// Placeholder for not-yet-used windows parameters.  Implemented for: [`()`](https://doc.rust-lang.org/std/primitive.unit.html)
pub trait Reserved : sealed::Reserved {}
impl Reserved for () {}

mod sealed {
    pub trait Reserved {}
    impl Reserved for () {}
}
