use winapi::shared::minwindef::*;

use std::io;



pub(crate) fn succeeded_to_result(succeeded: BOOL) -> io::Result<()> {
    match succeeded {
        0 => Err(io::Error::last_os_error()),
        _ => Ok(()),
    }
}

pub(crate) const fn size_of_32<T>() -> u32 {
    SizeOf32Impl::<T>::SIZE
}

pub(crate) const fn size_of_val_32_sized<T>(_: &T) -> u32 {
    SizeOf32Impl::<T>::SIZE
}

struct SizeOf32Impl<T>(T);
impl<T> SizeOf32Impl<T> {
    const SIZE : u32 = {
        let size = core::mem::size_of::<T>();
        assert!(size <= (u32::MAX as usize));
        size as _
    };
}
