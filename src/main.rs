#![feature(asm)]

mod syscall;
mod os;

// hako <cmd> <args>

/// Mapping of ids from child to the parent.
struct IdMapping {
	container: u32,
	host: u32,
	length: u32,
}

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

	let uid_mappings = [IdMapping {
		container: 0,
		host: 1000,
		length: 1
	}];

	match os::clone(syscall::CLONE_NEWUTS | syscall::CLONE_NEWUSER) {
		0 => {
			println!("\x1b[1;32msuccess\x1b[0m: we're in the forked process");

			// Change hostname so we know who we are.
			let result = unsafe {
				syscall::sethostname("container".as_ptr() as _, 9)
			};

			if result == -1 {
				println!("\x1b[1;33mwarning\x1b[0m: failed to set hostname");
			}

			// After this, we are root in the container.
			std::fs::write("/proc/self/uid_map", "0 1000 1\n").unwrap();
			dbg!(std::fs::read_to_string("/proc/self/uid_map")).unwrap();

			// Change root directory to the image one.
			let result = unsafe {
				syscall::chroot("/home/lyna/var/pods/archfs\0".as_ptr() as _)
			};

			if result == -1 {
				println!("\x1b[1;33mwarning\x1b[0m: failed to change root");
			}

			// Move to / to avoid issues with invalid path.
			let result = unsafe {
				syscall::chdir("/\0".as_ptr() as _)
			};

			if result == -1 {
				println!("\x1b[1;33mwarning\x1b[0m: failed to change directory");
			}

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
