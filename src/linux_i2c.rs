#![cfg(feature = "std")]

use nix::fcntl::{open, OFlag};
use nix::unistd::{read, write};
use std::error::Error;
use std::os::unix::io::{AsRawFd, OwnedFd};

use crate::i2c_access::I2cAccess;

pub struct LinuxI2c {
    fd: OwnedFd,
}

nix::ioctl_write_int_bad!(i2c_slave, 0x0703);

impl LinuxI2c {
    pub fn new(path: &str, adr: u8) -> Result<Self, Box<dyn Error>> {
        let fd = open(path, OFlag::O_RDWR, nix::sys::stat::Mode::empty())?;
        let mut i2c = LinuxI2c { fd };
        i2c.set_slave_address(adr)?;
        Ok(i2c)
    }

    pub fn set_slave_address(&mut self, adr: u8) -> Result<(), Box<dyn Error>> {
        unsafe {
            i2c_slave(self.fd.as_raw_fd(), adr as libc::c_int)?;
        }
        Ok(())
    }
}

impl I2cAccess for LinuxI2c {
    type Error = Box<dyn std::error::Error>;

    fn write(&mut self, data: &[u8]) -> Result<usize, Self::Error> {
        let len = write(&self.fd, data)?;
        Ok(len)
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let len = read(&self.fd, buf)?;
        Ok(len)
    }
}
