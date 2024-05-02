use crate::{Error, Session, ZNP};

use znp_types::command::ser;
use znp_types::command::ser::Command;
use znp_types::command::sys::Capability;
use znp_types::packet::Packet;

use std::io::Write;

use enumflags2::BitFlags;
use semver::Version;
use serialport::SerialPort;

impl Session for Box<dyn SerialPort> {
    fn send_command(&mut self, command: &impl ser::Command) -> Result<(), Error> {
        let packet = Packet::from_command(command).serialize();
        self.write_all(packet.as_slice()).map_err(Error::IO)?;

        Ok(())
    }

    fn recv_frame(&mut self) -> Result<Packet, Error> {
        let packet = Packet::from_reader(self).map_err(Error::Packet)?;
        Ok(packet)
    }
}

pub struct ZNPImpl {
    pub(crate) align_structs: bool,
    pub(crate) capabilities: enumflags2::BitFlags<Capability>,

    pub(crate) tty: Box<dyn SerialPort>,
}

impl Session for ZNPImpl {
    fn send_command(&mut self, command: &impl Command) -> Result<(), Error> {
        self.tty.send_command(command)
    }
    fn recv_frame(&mut self) -> Result<Packet, Error> { self.tty.recv_frame() }
}

impl ZNP for ZNPImpl {
    fn version(&self) -> Version { Version::new(3, 30, 0) }

    fn align_structs(&self) -> bool { self.align_structs }
    fn capabilities(&self) -> BitFlags<Capability> { self.capabilities }
}
