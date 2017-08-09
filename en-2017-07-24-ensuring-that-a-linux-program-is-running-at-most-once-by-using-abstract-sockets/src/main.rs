//! This program demonstrates a way of ensuring that a program on Linux is
//! running at most once using abstract sockets (Unix domain sockets and the
//! abstract namespace).
//!
//! For more details, see e.g. Section 57.6 ("The Linux Abstract Socket
//! Namespace") in The Linux Programming Interface by Michael Kerrisk.

// We use the 'nix' crate as it provides Rust-friendly bindings to various *nix
// system functions.
extern crate nix;

use std::error::Error;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use nix::sys::socket::AddressFamily;
use nix::sys::socket::SockAddr;
use nix::sys::socket::SockFlag;
use nix::sys::socket::SockType;
use nix::sys::socket::UnixAddr;
use nix::sys::socket::bind;
use nix::sys::socket::socket;

fn do_work() {
    // We simulate the actual work by sleeping.
    // Use e.g. Ctrl-C or SIGTERM to kill the program.
    sleep(Duration::from_secs(3600));
}

fn run() -> Result<(), Box<Error>> {
    // Create a Unix domain socket socket.
    let s = socket(AddressFamily::Unix, SockType::Stream, SockFlag::empty(), 0)?;

    // Create a socket address in the abstract namespace using a unique ID.
    let addr = SockAddr::Unix(UnixAddr::new_abstract(b"some-unique-id")?);

    // Try binding the socket to the above address in the abstract namespace.
    // When bind() fails with EADDRINUSE, it means that we should quit as
    // another instance of our program is already running.
    //
    // When the process is terminated, the kernel automatically closes the
    // socket.
    if let Err(e) = bind(s, &addr) {
        match e {
            nix::Error::Sys(nix::Errno::EADDRINUSE) => {
                eprintln!("program is already running");
                return Ok(());
            },
            _ => {
                // bind() failed because of an unexpected reason.
                return Err(Box::new(e));
            }

        };
    }

    do_work();

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {}", e);
        exit(1);
    }
}
