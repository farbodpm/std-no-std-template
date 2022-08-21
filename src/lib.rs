#![cfg_attr(not(feature = "std"), no_std)]
#![crate_name = "{{name}}"]
#![crate_type = "rlib"]

extern crate alloc;
use alloc::{boxed::Box, format, vec, vec::Vec};
