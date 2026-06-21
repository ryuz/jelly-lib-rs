#![allow(dead_code)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

pub mod i2c_hal;
pub mod imx219_sensor_driver;
pub mod video_dma_pac;
pub mod video_format_regularizer_pac;

#[cfg(all(feature = "std", target_os = "linux"))]
pub mod linux_i2c;



#[cfg(feature = "std")]
pub fn portable_delay(duration : core::time::Duration) {
    std::thread::sleep(duration);
}

#[cfg(not(feature = "std"))] 
pub fn portable_delay(duration: core::time::Duration) {
    let us = duration.as_micros() as u64;
    let iterations = us * 1000; // 概算
    for _ in 0..iterations {
        unsafe {
            core::arch::asm!("nop");
        }
    }
}

