#![allow(dead_code)]

use jelly_mem_access::MemAccess;
use crate::portable_delay;

// Video format regularizer
const REG_VIDEO_FMTREG_CORE_ID: usize = 0x00;
const REG_VIDEO_FMTREG_CORE_VERSION: usize = 0x01;
const REG_VIDEO_FMTREG_CTL_CONTROL: usize = 0x04;
const REG_VIDEO_FMTREG_CTL_STATUS: usize = 0x05;
const REG_VIDEO_FMTREG_CTL_INDEX: usize = 0x07;
const REG_VIDEO_FMTREG_CTL_SKIP: usize = 0x08;
const REG_VIDEO_FMTREG_CTL_FRM_TIMER_EN: usize = 0x0a;
const REG_VIDEO_FMTREG_CTL_FRM_TIMEOUT: usize = 0x0b;
const REG_VIDEO_FMTREG_PARAM_WIDTH: usize = 0x10;
const REG_VIDEO_FMTREG_PARAM_HEIGHT: usize = 0x11;
const REG_VIDEO_FMTREG_PARAM_FILL: usize = 0x12;
const REG_VIDEO_FMTREG_PARAM_TIMEOUT: usize = 0x13;



#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum VideoFormatRegularizerError {
    Timeout,
}

impl core::fmt::Display for VideoFormatRegularizerError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            VideoFormatRegularizerError::Timeout => write!(f, "DMA operation timeout"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for VideoFormatRegularizerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}


type Result<T> = core::result::Result<T, VideoFormatRegularizerError>;

pub struct VideoFormatRegularizer<T: MemAccess> {
    acc: T,
    running: bool,    
}


impl<T: MemAccess> VideoFormatRegularizer<T> {
    pub fn new(acc: T) -> Result<Self> {
        Ok(Self {
            acc: acc,
            running: false,
        })
    }

    pub fn start(&mut self) -> Result<()> {
        self.write_reg(REG_VIDEO_FMTREG_CTL_CONTROL, 3);
        self.running = true;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        self.running = false;
        self.write_reg(REG_VIDEO_FMTREG_CTL_CONTROL, 0);
        for _ in 0..100000 {
            let status = self.read_reg(REG_VIDEO_FMTREG_CTL_STATUS);
            if (status & 1) == 0 {
                return Ok(());
            }
            // wait
            portable_delay(core::time::Duration::from_micros(1));
        }
        Err(VideoFormatRegularizerError::Timeout)
    }

    pub fn set_image_size(&mut self, width : usize, height : usize) -> Result<()> {
        self.write_reg(REG_VIDEO_FMTREG_PARAM_WIDTH, width);
        self.write_reg(REG_VIDEO_FMTREG_PARAM_HEIGHT, height);
        if self.running {
            self.write_reg(REG_VIDEO_FMTREG_CTL_CONTROL, 3);
        }
        Ok(())
    }
    

    fn write_reg(&mut self, reg: usize, data: usize) {
        unsafe {
            self.acc.write_reg(reg, data);
        }
    }

    fn read_reg(&mut self, reg: usize) -> usize {
        unsafe { self.acc.read_reg(reg) }
    }

}


