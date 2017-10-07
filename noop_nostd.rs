// Note that binaries without stdlib currently require Rust nightly.
// See: https://doc.rust-lang.org/1.20.0/book/first-edition/using-rust-without-the-standard-library.html
#![feature(start)]
#![no_std]

extern crate libc;

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    42
}
