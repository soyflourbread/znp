use crate::{Error, ZNP};

pub trait NVRam: ZNP {
    fn nv_exists(&mut self) -> bool;
    fn nv_len(&mut self) -> u8;
    fn nv_read(&mut self) -> Result<Vec<u8>, Error>;
}
