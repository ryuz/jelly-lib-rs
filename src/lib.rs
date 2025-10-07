#![allow(dead_code)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

pub mod i2c_access;
pub mod imx219_control;

#[cfg(feature = "std")]
pub mod linux_i2c;
