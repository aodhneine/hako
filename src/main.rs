#![feature(asm)]

mod syscall;
mod os;

// hako run <cmd> <args>

fn main() {
	println!("running as pid {}", syscall::getpid());

	match os::clone(0) {
		0 => {
			println!("\x1b[1;32msuccess\x1b[0m: we're in the forked process");

			let mut argv = [b"/bin/ls\0".as_ptr() as _, core::ptr::null()];

			let mut envp = [
				b"HOME=/home/lyna\0".as_ptr() as _,
				b"PATH=/usr/local/sbin:/usr/local/bin:/usr/bin:/usr/bin/site_perl:/usr/bin/vendor_perl:/usr/bin/core_perl\0".as_ptr() as _,
				b"TERM=alacritty\0".as_ptr() as _,
				core::ptr::null(),
			];

			let result = unsafe {
				syscall::execve(b"/bin/bash\0".as_ptr() as _, argv.as_mut_ptr(), envp.as_mut_ptr())
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
