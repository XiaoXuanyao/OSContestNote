const SYSCALL_EXIT: usize = 93;

#[allow(dead_code)]
pub fn sys_exit(xstate: i32) -> isize {
    crate::syscall::syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}
