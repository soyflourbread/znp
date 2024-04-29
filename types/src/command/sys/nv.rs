use super::SUBSYS;
use crate::command::CommandType::*;
use crate::command::{de, ser, to_bincode_config, Command, CommandID, CommandType, Status};
use log::debug;
use num_traits::FromPrimitive;

use crate::command::de::Error;
use znp_macros::{Command, PassRsp};

#[derive(Command, bincode::Encode, Debug, Clone)]
#[cmd(req_type = "SREQ", rsp_type = "SRSP", subsys = "SUBSYS", id = 0x33)]
pub struct NVLength {
    sys_id: u8,
    item_id: u16,
    sub_id: u16,
}

impl NVLength {
    pub fn new(sys_id: u8, item_id: u16, sub_id: u16) -> Self {
        Self {
            sys_id,
            item_id,
            sub_id,
        }
    }
}

impl ser::Command for NVLength {
    fn len(&self) -> u8 { 5 }
    fn data(&self) -> Vec<u8> { bincode::encode_to_vec(self, to_bincode_config()).unwrap() }
}

impl de::Command for NVLength {
    type Output = u8;
    fn to_output(&self, data_frame: Vec<u8>) -> Result<Self::Output, Error> {
        if data_frame.len() != 1 {
            return Err(Error::UnexpectedEOF);
        }
        Ok(data_frame[0])
    }
}

#[derive(Command, bincode::Encode, Debug, Clone)]
#[cmd(req_type = "SREQ", rsp_type = "SRSP", subsys = "SUBSYS", id = 0x33)]
pub struct NVRead {
    sys_id: u8,
    item_id: u16,
    sub_id: u16,
    offset: u16,
    length: u8,
}

impl NVRead {
    pub fn new(sys_id: u8, item_id: u16, sub_id: u16, offset: u16, length: u8) -> Self {
        Self {
            sys_id,
            item_id,
            sub_id,
            offset,
            length,
        }
    }
}

impl ser::Command for NVRead {
    fn len(&self) -> u8 { 8 }
    fn data(&self) -> Vec<u8> { bincode::encode_to_vec(self, to_bincode_config()).unwrap() }
}

impl de::Command for NVRead {
    type Output = (Status, Vec<u8>);
    fn to_output(&self, mut data_frame: Vec<u8>) -> Result<Self::Output, Error> {
        if data_frame.len() < 2 {
            return Err(Error::UnexpectedEOF);
        }
        let Some(status) = Status::from_u8(data_frame[0]) else {
            return Err(Error::Unknown);
        };
        let len = data_frame[1] as usize;
        let data_frame = data_frame.drain(2..).collect::<Vec<_>>();
        if data_frame.len() != len {
            debug!(
                "nv read frame length mismatch, expected={:?}, actual={:?}",
                len,
                data_frame.len()
            );
            return Err(Error::UnexpectedEOF);
        }
        Ok((status, data_frame))
    }
}
