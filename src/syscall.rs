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
unsafe fn syscall1(n: i64, a: i64) -> i64 {
	let mut ret: i64;
	asm!("syscall", inout("rax") n => ret, in("rdi") a, lateout("rcx") _, lateout("r11") _);
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

#[inline]
unsafe fn syscall5(n: i64, a: i64, b: i64, c: i64, d: i64, e: i64) -> i64 {
	let mut ret: i64;

	asm!(
		"syscall",
		inout("rax") n => ret,
		in("rdi") a,
		in("rsi") b,
		in("rdx") c,
		in("r10") d,
		in("r8") e,
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
	Chdir = 80,
	Setuid = 105,
	Setgid = 106,
	Chroot = 161,
	Mount = 165,
	Sethostname = 170,
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

#[inline]
pub unsafe fn chdir(path: *const i8) -> i32 {
	return syscall1(Syscall::Chdir as i64, path as _) as i32;
}

#[inline]
pub unsafe fn setuid(uid: i32) -> i32 {
	return syscall1(Syscall::Setuid as i64, uid as _) as i32;
}

#[inline]
pub unsafe fn setgid(uid: i32) -> i32 {
	return syscall1(Syscall::Setgid as i64, uid as _) as i32;
}

#[inline]
pub unsafe fn chroot(path: *const i8) -> i32 {
	return syscall1(Syscall::Chroot as i64, path as _) as i32;
}

#[inline]
pub unsafe fn sethostname(name: *const i8, len: usize) -> i32 {
	return syscall2(Syscall::Sethostname as i64, name as _, len as _) as i32;
}

// -=- mount() related stuff. -=-

pub const MS_REC: u64 = 16384;
pub const MS_PRIVATE: u64 = 1 << 18;

#[inline]
pub unsafe fn mount(source: *const i8, target: *const i8, fstype: *const i8, flags: u64, data: *const std::ffi::c_void) -> i32 {
	return syscall5(Syscall::Mount as i64, source as _, target as _, fstype as _, flags as _, data as _) as i32;
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
	pub ru_utime: timeval,
	pub ru_stime: timeval,
	pub ru_maxrss: i64,
	pub ru_ixrss: i64,
	pub ru_idrss: i64,
	pub ru_isrss: i64,
	pub ru_minflt: i64,
	pub ru_majlft: i64,
	pub ru_nswap: i64,
	pub ru_inblock: i64,
	pub ru_oublock: i64,
	pub ru_msgsnd: i64,
	pub ru_msgrcv: i64,
	pub ru_nsignals: i64,
	pub ru_nvcsw: i64,
	pub ru_nivcsw: i64,
}

#[inline]
pub unsafe fn wait4(pid: i32, wstatus: *mut i32, options: i32, rusage: *mut rusage) -> i32 {
	return syscall4(Syscall::Wait4 as i64, pid as _, wstatus as _, options as _, rusage as _) as i32;
}

// -=- clone3() related stuff. -=-

pub const CLONE_NEWNS: u64 = 0x00020000;
pub const CLONE_NEWUTS: u64 = 0x04000000;
pub const CLONE_NEWPID: u64 = 0x20000000;
pub const CLONE_NEWUSER: u64 = 0x10000000;

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
