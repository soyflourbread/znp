use log::{debug, error, info};

use znp_macros::{Command, EmptyReq};

use super::{de, ser, Command, CommandID, Subsystem};
use super::{CommandType, CommandType::*};

const SUBSYS: Subsystem = Subsystem::IFaceSYS;

/// Capability of the device, 2 bytes.
/// See Z-stack Monitor and Test API, 3.8.1.2.
#[enumflags2::bitflags]
#[repr(u16)]
#[derive(Debug, Copy, Clone)]
pub enum Capability {
    SYS = 0x0001,
    MAC = 0x0002,
    NWK = 0x0004,
    AF = 0x0008,
    ZDO = 0x0010,
    SAPI = 0x0020,
    UTIL = 0x0040,
    DEBUG = 0x0080,
    APP = 0x0100,
    ZOAD = 0x1000,
}

#[derive(Command, EmptyReq, Debug, Clone)]
#[cmd(req_type = "SREQ", rsp_type = "SRSP", subsys = "SUBSYS", id = 0x01)]
pub struct Ping {}

impl de::Command for Ping {
    type Output = enumflags2::BitFlags<Capability>;
    fn to_output(&self, data_frame: Vec<u8>) -> Result<Self::Output, de::Error> {
        if data_frame.len() != 2 {
            debug!(
                "data frame length mismatch, expected={}, actual={}",
                2,
                data_frame.len()
            );
            return Err(de::Error::UnexpectedEOF);
        }
        let mut ret = u16::MIN;
        ret |= data_frame[0] as u16;
        ret <<= 8;
        ret |= data_frame[1] as u16;
        debug!("recv capabilities bitflag: {}", ret);
        let ret = Self::Output::from_bits_truncate(ret);
        Ok(ret)
    }
}
