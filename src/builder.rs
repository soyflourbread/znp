use semver::Version;
use std::time::Duration;

use serialport::{DataBits, StopBits};

use znp_types::command::sys::{ExNvIds, NVLength, NvSysIds, Ping, NVID};
use znp_types::command::util::AssocFindDevice;

use crate::{Error, Session, ZNPImpl, ZNP};

pub struct Builder {
    port: String,
}

impl Builder {
    pub fn from_port(port: String) -> Self { Self { port } }

    pub fn connect(self) -> Result<impl ZNP, Error> {
        let mut tty = serialport::new(self.port, 115200)
            .data_bits(DataBits::Eight)
            .stop_bits(StopBits::One)
            .open()
            .map_err(Error::TTY)?;
        // reset ZNP board to skip bootloader
        for (dtr, rts) in [(false, false), (false, true), (false, false)] {
            tty.write_data_terminal_ready(dtr).map_err(Error::TTY)?;
            tty.write_request_to_send(rts).map_err(Error::TTY)?;
        }
        std::thread::sleep(Duration::from_millis(150));

        // clear tty
        let clr = vec![0x10u8 ^ 0xFF; 256];
        tty.write_all(clr.as_slice()).map_err(Error::IO)?;
        std::thread::sleep(Duration::from_millis(2500));

        let capabilities = tty.request(&Ping::default())?;
        let device = tty.request(&AssocFindDevice::new(0))?;
        let align_structs = match device.len() {
            28 => false,
            36 => true,
            _ => return Err(Error::Unknown),
        };

        let mut version = Version::new(3, 30, 0);
        if let Err(crate::Error::CommandNotFound) = tty.request(&NVLength::new(NVID::new(
            NvSysIds::ZStack as u8,
            ExNvIds::TClkTable as u16,
            0,
        ))) {
            version = Version::new(3, 0, 0);
        }

        let ret = ZNPImpl {
            version,
            align_structs,
            capabilities,
            tty,
        };
        Ok(ret)
    }
}
