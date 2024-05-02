use znp_types::command::sys::Capability;
use znp_types::command::{de, ser};
use znp_types::packet::{self, Packet};

use std::time::Duration;

use semver::Version;

mod builder;
pub use builder::Builder;
mod imple;
mod nv;

use imple::ZNPImpl;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unknown error")]
    Unknown,

    #[error("tty error: {0:?}")]
    TTY(serialport::Error),
    #[error("I/O error: {0:?}")]
    IO(std::io::Error),
    #[error("packet error: {0:?}")]
    Packet(packet::Error),
}

pub trait Session {
    fn send_command(&mut self, command: &impl ser::Command) -> Result<(), Error>;
    fn recv_frame(&mut self) -> Result<Packet, Error>;
    fn request<C: ser::Command + de::Command>(&mut self, command: &C) -> Result<C::Output, Error> {
        self.send_command(command)?;
        loop {
            let mut exec_fn = || -> Result<_, Box<dyn std::error::Error>> {
                let frame = self.recv_frame()?;
                let ret = command.deserialize(frame.command)?;
                Ok(ret)
            };
            if let Ok(ret) = exec_fn() {
                return Ok(ret);
            }
            std::thread::sleep(Duration::from_millis(500));
        }
    }
}

pub trait ZNP: Session {
    fn version(&self) -> Version;

    fn align_structs(&self) -> bool;
    fn capabilities(&self) -> enumflags2::BitFlags<Capability>;
}
