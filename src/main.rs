#![feature(asm)]

mod syscall;
mod os;

// hako run <cmd> <args>

fn main() {
	println!("running as pid {}", syscall::getpid());

	let mut args = std::env::args();
	// We ignore the first argument, which is canonically the binary name, and is
	// pretty much useless to us.
	args.next();

	let command = match args.next() {
		Some(s) => s,
		None => {
			println!("\x1b[1;31mfailure\x1b[0m: expected a command name");
			return;
		},
	};

	let cmd = match std::ffi::CString::new(command) {
		Ok(cs) => cs,
		Err(_) => {
			println!("\x1b[1;31mfailure\x1b[0m: command name contains NUL byte, which is not allowed");
			return;
		},
	};

	// The rest of the arguments are going to be passed to the command.
	let arguments = args.map(|s| std::ffi::CString::new(s))
		.flatten()
		.collect::<Vec<_>>();

	match os::clone(0) {
		0 => {
			println!("\x1b[1;32msuccess\x1b[0m: we're in the forked process");

			let mut args = Vec::new();
			args.push(cmd.as_ptr());
			args.extend(arguments.iter().map(|s| s.as_ptr()));
			args.push(core::ptr::null());

			let mut envp = [
				b"HOME=/home/lyna\0".as_ptr() as _,
				b"PATH=/usr/local/sbin:/usr/local/bin:/usr/bin:/usr/bin/site_perl:/usr/bin/vendor_perl:/usr/bin/core_perl\0".as_ptr() as _,
				b"TERM=alacritty\0".as_ptr() as _,
				core::ptr::null(),
			];

			let result = unsafe {
				syscall::execve(cmd.as_ptr(), args.as_mut_ptr(), envp.as_mut_ptr())
			};

			if result == -1 {
				println!("\x1b[1;31mfailure\x1b[0m: failed to call execve");
				return;
			}
		},
		c if c < 0 => {
			println!("\x1b[1;31mfailure\x1b[0m: failed to spawn a process");
			return;
		},
		c => {
			println!("\x1b[2minfo\x1b[0m: forked child with pid {}", c);

			let mut status = 0;

			let pid = unsafe {
				syscall::wait4(c, &mut status as *mut i32, 0, std::ptr::null_mut())
			};

			println!("\x1b[1;32msuccess\x1b[0m: child with pid {} exited", pid);
		},
	};
}
