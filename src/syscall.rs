#[inline]
unsafe fn syscall0(n: i64) -> i64 {
	let mut ret: i64;
	// Mark rcx and r11 as clobbered, as syscall opcode uses rcx to store the
	// address of the next instruction, and r11 to store the value of rflags
	// register,
	asm!("syscall", inout("rax") n => ret, lateout("rcx") _, lateout("r11") _);
	return ret;
}

#[inline]
unsafe fn syscall2(n: i64, a: i64, b: i64) -> i64 {
	let mut ret: i64;

	asm!(
		"syscall",
		inout("rax") n => ret,
		in("rdi") a,
		in("rsi") b,
		lateout("rcx") _,
		lateout("r11") _,
	);

	return ret;
}

#[inline]
unsafe fn syscall3(n: i64, a: i64, b: i64, c: i64) -> i64 {
	let mut ret: i64;

	asm!(
		"syscall",
		inout("rax") n => ret,
		in("rdi") a,
		in("rsi") b,
		in("rdx") c,
		lateout("rcx") _,
		lateout("r11") _,
	);

	return ret;
}

#[inline]
unsafe fn syscall4(n: i64, a: i64, b: i64, c: i64, d: i64) -> i64 {
	let mut ret: i64;

	asm!(
		"syscall",
		inout("rax") n => ret,
		in("rdi") a,
		in("rsi") b,
		in("rdx") c,
		in("r10") d,
		lateout("rcx") _,
		lateout("r11") _,
	);

	return ret;
}

#[repr(i64)]
#[non_exhaustive]
pub enum Syscall {
	Getpid = 39,
	Execve = 59,
	Wait4 = 61,
	Clone3 = 435,
}

#[inline]
pub fn getpid() -> i32 {
	// SAFETY: Calling getpid is never supposed to fail.
	return unsafe {
		syscall0(Syscall::Getpid as i64) as i32
	};
}


/// # Safety
/// This is **VERY VERY VERY** unsafe. Provide invalid pointer to argv or envp
/// and anything can happen. Forget to terminate argv or envp with null pointer
/// and anything can happen. Forget to terminate strings in argv or envp with
/// a NUL byte and anything can happen.
#[inline]
pub unsafe fn execve(pathname: *const i8, argv: *mut *const i8, envp: *mut *const i8) -> i32 {
	return syscall3(Syscall::Execve as i64, pathname as _, argv as _, envp as _) as i32;
}

// -=- wait4() related stuff. -=-

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct timeval {
	tv_sec: i64,
	tv_usec: i64,
}

// /usr/include/bits/types/struct_rusage.h
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct rusage {
	ru_utime: timeval,
	ru_stime: timeval,
	ru_maxrss: i64,
	ru_ixrss: i64,
	ru_idrss: i64,
	ru_isrss: i64,
	ru_minflt: i64,
	ru_majlft: i64,
	ru_nswap: i64,
	ru_inblock: i64,
	ru_oublock: i64,
	ru_msgsnd: i64,
	ru_msgrcv: i64,
	ru_nsignals: i64,
	ru_nvcsw: i64,
	ru_nivcsw: i64,
}

#[inline]
pub unsafe fn wait4(pid: i32, wstatus: *mut i32, options: i32, rusage: *mut rusage) -> i32 {
	return syscall4(Syscall::Wait4 as i64, pid as _, wstatus as _, options as _, rusage as _) as i32;
}

// -=- clone3() related stuff. -=-

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct clone_args {
	pub flags: u64,
	pub pidfd: u64,
	pub child_tid: u64,
	pub parent_tid: u64,
	pub exit_signal: u64,
	pub stack: u64,
	pub stack_size: u64,
	pub tls: u64,
	pub set_tid: u64,
	pub set_tid_size: u64,
	pub cgroup: u64,
}

#[inline]
pub unsafe fn clone3(args: *const clone_args, size: usize) -> i64 {
	debug_assert!(size == std::mem::size_of::<clone_args>());
	return syscall2(Syscall::Clone3 as i64, args as _, size as _);
}
