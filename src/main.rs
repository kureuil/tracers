extern crate nix;

use std::ffi::{CString, NulError};
use nix::libc::pid_t;
use std::convert::From;
use nix::unistd::{fork, ForkResult, execvp};
use nix::sys::wait::{waitpid};

/// TracersError is the error type used in this crate.
#[derive(Debug)]
struct TracersError;

/// Allow creating a `TracersError` from a `nix::Error`.
impl From<nix::Error> for TracersError {
	fn from(_: nix::Error) -> TracersError {
		TracersError {}
	}
}

/// Allow creating a `TracersError` from a `std::ffi::NulError`.
impl From<NulError> for TracersError {
	fn from(_: NulError) -> TracersError {
		TracersError {}
	}
}

/// Alias Result with the TracersError type.
type Result<T> = std::result::Result<T, TracersError>;

/// Traces the process whose ID is `child`.
fn trace(child: pid_t) -> Result<()> {
	try!(waitpid(child, None));
	Ok(())
}

/// Executes the command represented by the vector of String.
fn exec(args: Vec<String>) -> Result<()> {
	let filepath = try!(CString::new(args[0].clone()));
	let mut converted: Vec<CString> = Vec::with_capacity(args.len());
	for arg in args {
		converted.push(try!(CString::new(arg)));
	}
	try!(execvp(&filepath, &converted));
	Ok(())
}

/// Entry point for the program taking the arguments in parameter.
fn start(args: Vec<String>) -> Result<()> {
	match try!(fork()) {
		ForkResult::Parent { child } => try!(trace(child)),
		ForkResult::Child => try!(exec(args)),
	};
    Ok(())
}

fn main() {
	let args: Vec<_> = std::env::args().skip(1).collect();
	if let Err(e) = start(args) {
		println!("An error occurred: {:?}", e);
	}
}
