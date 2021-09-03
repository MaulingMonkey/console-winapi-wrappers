use winapi::shared::minwindef::*;

use std::io;



pub(crate) fn succeeded_to_result(succeeded: BOOL) -> io::Result<()> {
    match succeeded {
        0 => Err(io::Error::last_os_error()),
        _ => Ok(()),
    }
}
