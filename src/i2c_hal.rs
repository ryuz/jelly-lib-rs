use core::result::Result;


/// I2C Hardware Abstract Layer trait
pub trait I2cHal {
    type Error;
    fn write(&mut self, data: &[u8]) -> Result<usize, Self::Error>;
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error>;
}
