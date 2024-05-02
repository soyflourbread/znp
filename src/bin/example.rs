use serialport::SerialPortType;

use znp::{Builder, Session, ZNP};
use znp_types::command::sys::{ExNvIds, NVLength, NvSysIds, NVID};

fn get_first_usb_serial() -> String {
    let ports = serialport::available_ports().unwrap();
    for port in ports {
        if let SerialPortType::UsbPort(_) = port.port_type {
            return port.port_name;
        }
    }
    String::new()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let port = get_first_usb_serial();
    let port = std::env::var("ZNP_SERIAL").unwrap();
    let mut controller = Builder::from_port(port).connect()?;
    println!(
        "caps: {}, align: {}",
        controller.capabilities(),
        controller.align_structs()
    );

    let length = controller.request(&NVLength::new(NVID::new(
        NvSysIds::ZStack as u8,
        ExNvIds::TClkTable as u16,
        0,
    )))?;
    println!("length: {}", length);

    Ok(())
}
