use super::result::*;

#[inline(always)]
pub fn syscall0(n: usize) -> crate::Result {
    let syscall_return: usize;
    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") n => syscall_return,
            out("rcx") _,
            out("r11") _,
        );
    }
    handle_result(syscall_return)
}

pub mod ok {
    macros::result!(
        Ok;
        "Human Ok";
        usize;
        [
            [0; X86_64_SYSCALL0_OK; Default; usize; "ZE"; "Entry to ze"],
        ]
    );

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Default(no)
        }
    }
}

pub mod error {
    macros::result!(
        Error;
        "Human error";
        usize;
        [
            [0; X86_64_SYSCALL0_ERROR; Default; usize; "ZE"; "Entry to ze"],
        ]
    );

    impl Error {
        pub fn from_no(no: usize) -> Self {
            Error::Default(no)
        }
    }
}

pub use error::Error;
pub use ok::Ok;

pub type Result = core::result::Result<Ok, Error>;

pub fn handle_result(result: usize) -> crate::Result {
    if (result as isize) < 0 {
        Err(crate::Error::Arch(crate::arch::Error::X86_64Syscall(
            crate::arch::syscall::Error::X86_64Syscall0(Error::Default(result)),
        )))
    } else {
        Ok(crate::Ok::Arch(crate::arch::Ok::X86_64Syscall(
            crate::arch::syscall::Ok::X86_64Syscall0(Ok::Default(result)),
        )))
    }
}
