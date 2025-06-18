#![allow(dead_code)]
#![allow(unused_imports)]

pub mod asserts;

#[cfg(target_arch = "x86_64")]
pub mod raw_machine_code;
