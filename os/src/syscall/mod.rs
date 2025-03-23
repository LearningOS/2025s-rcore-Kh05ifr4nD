//! Implementation of syscalls
//!
//! The single entry point to all system calls, [`syscall()`], is called
//! whenever userspace wishes to perform a system call using the `ecall`
//! instruction. In this case, the processor raises an 'Environment call from
//! U-mode' exception, which is handled as one of the cases in
//! [`crate::trap::trap_handler`].
//!
//! For clarity, each single syscall is implemented as its own function, named
//! `sys_` then the name of the syscall. You can find functions like this in
//! submodules, and you should also implement syscalls this way.
//!

use strum::{EnumCount, FromRepr};

use crate::task::TASK_MANAGER;
mod fs;
mod process;

/// System call
#[derive(Debug, Clone, Copy, EnumCount, FromRepr)]
#[repr(usize)]
pub enum Syscall {
    /// Exit
    Exit = 93,
    /// Get time
    GetTime = 169,
    /// Get task information
    Trace = 410,
    /// Write
    Write = 64,
    /// Yield
    Yield = 124,
}
impl Syscall {
    /// Get the number of syscalls
    pub const fn idx(&self) -> usize {
        match self {
            Syscall::Exit => 0,
            Syscall::GetTime => 1,
            Syscall::Trace => 2,
            Syscall::Write => 3,
            Syscall::Yield => 4,
        }
    }
}

/// handle syscall exception with `syscall_id` and other arguments
pub fn syscall(syscall: Syscall, args: [usize; 3]) -> isize {
    use process::{sys_exit, sys_get_time, sys_trace, sys_yield, TimeVal};
    TASK_MANAGER.inc_cur_syscall_time(syscall);
    match syscall {
        Syscall::Exit => sys_exit(args[0] as i32),
        Syscall::GetTime => sys_get_time(args[0] as *mut TimeVal, args[1]),
        Syscall::Trace => sys_trace(args[0], args[1], args[2]),
        Syscall::Write => fs::sys_write(args[0], args[1] as *const u8, args[2]),
        Syscall::Yield => sys_yield(), // _ => panic!("Unsupported syscall_id: {:?}", syscall),
    }
}
