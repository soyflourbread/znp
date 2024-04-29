use log::debug;

use znp_macros::{Command, EmptyReq};

use crate::command::CommandType::*;
use crate::command::{de, ser, to_bincode_config, Command, CommandID, CommandType};

use super::SUBSYS;

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

#[derive(Command, EmptyReq, Default, Debug, Clone)]
#[cmd(req_type = "SREQ", rsp_type = "SRSP", subsys = "SUBSYS", id = 0x01)]
pub struct Ping {}

impl de::Command for Ping {
    type Output = enumflags2::BitFlags<Capability>;
    fn to_output(&self, data_frame: Vec<u8>) -> Result<Self::Output, de::Error> {
        #[derive(bincode::Decode)]
        struct Rsp {
            capabilities: u16,
        }
        let (rsp, len): (Rsp, usize) =
            bincode::decode_from_slice(data_frame.as_slice(), to_bincode_config())
                .map_err(|_| de::Error::UnexpectedEOF)?;
        if len != data_frame.len() {
            debug!(
                "data frame length mismatch, expected={}, actual={}",
                len,
                data_frame.len()
            );
            return Err(de::Error::UnexpectedEOF);
        }
        debug!("recv capabilities bitflag: {}", rsp.capabilities);
        Ok(Self::Output::from_bits_truncate(rsp.capabilities))
    }
}