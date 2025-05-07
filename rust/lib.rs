#![no_std]

extern crate error;

use core::result::Result as CoreResult;
use error::Error;

pub type Result<T> = CoreResult<T, Error>;

pub fn real_main(_argc: i32, _argv: *const *const i8) -> i32 {
    0
}
