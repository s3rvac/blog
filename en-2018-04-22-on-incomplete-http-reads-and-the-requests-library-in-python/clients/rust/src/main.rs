//!
//! An HTTP client in Rust for verifying the reading of incomplete responses.
//!

#[macro_use]
extern crate failure;
extern crate reqwest;

use failure::Error;
use failure::Fail;
use failure::ResultExt;
use std::io::Read;

fn run() -> std::result::Result<(), Error> {
    let mut response = reqwest::get("http://localhost:8080")
        .context("failed to send request")?;

    if !response.status().is_success() {
        let err = format_err!("request failed with HTTP {}", response.status());
        return Err(err);
    }

    let mut contents = Vec::new();
    response.read_to_end(&mut contents)
        .context("failed to read the contents of the response")?;

    println!("{:?}", response.headers());
    println!("{:?}", contents);
    println!("{}", contents.len());
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {}", err);

        let mut fail: &Fail = err.cause();
        while let Some(cause) = fail.cause() {
            eprintln!("  cause: {}", cause);
            fail = cause;
        }

        std::process::exit(1);
    }
}
