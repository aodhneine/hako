use crate::syscall;

const SIGCHLD: u64 = 17;

pub fn clone(flags: u64) -> i32 {
	let args = syscall::clone_args {
		flags,
		pidfd: 0,
		child_tid: 0,
		parent_tid: 0,
		exit_signal: SIGCHLD,
		stack: 0,
		stack_size: 0,
		tls: 0,
		set_tid: 0,
		set_tid_size: 0,
		cgroup: 0,
	};

	return unsafe {
		syscall::clone3(&args as *const syscall::clone_args, 88) as i32
	};
}
