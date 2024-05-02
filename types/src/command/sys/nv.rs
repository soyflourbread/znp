use crate::command::{
    de, deserialize_bincode, ser, serialize_bincode, Command, CommandID, CommandType, Status,
};

use znp_macros::Command;

use num_traits::FromPrimitive;

use log::debug;

use super::SUBSYS;

#[derive(bincode::Encode, Debug, Clone, Copy)]
pub struct NVID {
    sys_id: u8,
    item_id: u16,
    sub_id: u16,
}

impl NVID {
    pub fn new(sys_id: u8, item_id: u16, sub_id: u16) -> Self {
        Self {
            sys_id,
            item_id,
            sub_id,
        }
    }
}

#[repr(u8)]
#[derive(bincode::Encode, Debug, Clone, Copy)]
pub enum NvSysIds {
    NvDrvr = 0,
    ZStack = 1,
    TiMac = 2,
    Remoti = 3,
    Ble = 4,
    _6Mesh = 5,
    TiOp = 6,
    App = 7,
}

#[repr(u16)]
#[derive(bincode::Encode, Debug, Clone, Copy)]
pub enum ExNvIds {
    Legacy = 0x0000,
    AddrMgr = 0x0001,
    BindingTable = 0x0002,
    DeviceList = 0x0003,
    TClkTable = 0x0004,
    TClkIcTable = 0x0005,
    ApsKeyDataTable = 0x0006,
    NwkSecMaterialTable = 0x0007,
}

#[derive(Command, bincode::Encode, Debug, Clone)]
#[cmd(subsys = "SUBSYS", id = 0x32)]
pub struct NVLength {
    id: NVID,
}

impl NVLength {
    pub fn new(id: NVID) -> Self { Self { id } }
}

impl ser::Command for NVLength {
    const REQUEST_TYPE: CommandType = CommandType::SREQ;
    fn len(&self) -> u8 { 5 }
    fn data(&self) -> Vec<u8> { serialize_bincode(self) }
}

impl de::Command for NVLength {
    const RESPONSE_TYPE: CommandType = CommandType::SRSP;
    type Output = u32;
    fn to_output(&self, data_frame: Vec<u8>) -> Result<Self::Output, de::Error> {
        #[derive(bincode::Decode)]
        struct Rsp {
            length: u32,
        }
        let rsp: Rsp = deserialize_bincode(data_frame)?;
        Ok(rsp.length)
    }
}

#[derive(Command, bincode::Encode, Debug, Clone)]
#[cmd(subsys = "SUBSYS", id = 0x33)]
pub struct NVRead {
    id: NVID,
    offset: u16,
    length: u8,
}

impl NVRead {
    pub fn new(id: NVID, offset: u16, length: u8) -> Self { Self { id, offset, length } }
}

impl ser::Command for NVRead {
    const REQUEST_TYPE: CommandType = CommandType::SREQ;
    fn len(&self) -> u8 { 8 }
    fn data(&self) -> Vec<u8> { serialize_bincode(self) }
}

impl de::Command for NVRead {
    const RESPONSE_TYPE: CommandType = CommandType::SRSP;
    type Output = (Status, Vec<u8>);
    fn to_output(&self, mut data_frame: Vec<u8>) -> Result<Self::Output, de::Error> {
        if data_frame.len() < 2 {
            return Err(de::Error::UnexpectedEOF);
        }
        let Some(status) = Status::from_u8(data_frame[0]) else {
            return Err(de::Error::Unknown);
        };
        let len = data_frame[1] as usize;
        let data_frame = data_frame.drain(2..).collect::<Vec<_>>();
        if data_frame.len() != len {
            debug!(
                "nv read frame length mismatch, expected={:?}, actual={:?}",
                len,
                data_frame.len()
            );
            return Err(de::Error::UnexpectedEOF);
        }
        Ok((status, data_frame))
    }
}
