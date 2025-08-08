pub mod flags;

pub use flags::*;

use super::Number;
use arch::{Arch, Callable};

static NUMBER: usize = Number::MMap as usize;

use result::define_error;

define_error!(
    "mmap",
    [[
        NotReadable,
        13,
        "File not open for reading",
        "EACCES",
        EACCES
    ]]
);

pub fn handle_result(arch_result: arch::Result) -> crate::Result {
    match arch_result {
        Err(arch::Error::TODO) => Err(crate::Error::MMap(Error::TODO)),
        Ok(no) => match no {
            _ => Ok((no, no)),
        },
    }
}

#[inline(always)]
pub fn mmap(
    addr: *mut u8,
    length: usize,
    prot: i32,
    flags: i32,
    fd: i32,
    offset: i64,
) -> crate::Result {
    let arch_result = Arch::syscall6(
        NUMBER,
        addr as usize,
        length,
        prot as usize,
        flags as usize,
        fd as usize,
        offset as usize,
    );

    handle_result(arch_result)
}
