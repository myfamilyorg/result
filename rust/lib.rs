#![no_std]

extern crate error;

use core::result::Result as CoreResult;
use error::Error;

pub type Result<T> = CoreResult<T, Error>;
