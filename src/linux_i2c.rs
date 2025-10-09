#![cfg(feature = "std")]

use nix::fcntl::{open, OFlag};
use nix::unistd::{read, write};
use std::os::unix::io::{AsRawFd, OwnedFd};

use crate::i2c_access::I2cAccess;

#[derive(Debug)]
pub enum LinuxI2cError {
    Nix(nix::Error),
    Io(std::io::Error),
}

impl From<nix::Error> for LinuxI2cError {
    fn from(error: nix::Error) -> Self {
        LinuxI2cError::Nix(error)
    }
}

impl From<std::io::Error> for LinuxI2cError {
    fn from(error: std::io::Error) -> Self {
        LinuxI2cError::Io(error)
    }
}

impl std::fmt::Display for LinuxI2cError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LinuxI2cError::Nix(e) => write!(f, "Nix error: {}", e),
            LinuxI2cError::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for LinuxI2cError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LinuxI2cError::Nix(e) => Some(e),
            LinuxI2cError::Io(e) => Some(e),
        }
    }
}

pub struct LinuxI2c {
    fd: OwnedFd,
}

nix::ioctl_write_int_bad!(i2c_slave, 0x0703);

impl LinuxI2c {
    pub fn new(path: &str, adr: u8) -> Result<Self, LinuxI2cError> {
        let fd = open(path, OFlag::O_RDWR, nix::sys::stat::Mode::empty())?;
        let mut i2c = LinuxI2c { fd };
        i2c.set_slave_address(adr)?;
        Ok(i2c)
    }

    pub fn set_slave_address(&mut self, adr: u8) -> Result<(), LinuxI2cError> {
        unsafe {
            i2c_slave(self.fd.as_raw_fd(), adr as libc::c_int)?;
        }
        Ok(())
    }
}

impl I2cAccess for LinuxI2c {
    type Error = LinuxI2cError;

    fn write(&mut self, data: &[u8]) -> Result<usize, Self::Error> {
        let len = write(&self.fd, data)?;
        Ok(len)
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let len = read(&self.fd, buf)?;
        Ok(len)
    }
}
