#![cfg_attr(feature = "no-std", no_std)]
#![crate_name = "{{name}}"]
#![crate_type = "rlib"]

#[cfg(feature = "no-std")]
extern crate alloc;
#[cfg(feature = "no-std")]
use alloc::{boxed::Box, format, vec, vec::Vec};

#[cfg(not(feature = "no-std"))]
use std::{boxed::Box, format, vec, vec::Vec};
