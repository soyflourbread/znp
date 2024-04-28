use serialport::SerialPortType;

use znp::{Builder, ZNP};

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
    let port = "/dev/tty.usbserial-110".to_string();
    let mut controller = Builder::from_port(port).connect()?;
    println!(
        "caps: {}, align: {}",
        controller.capabilities(),
        controller.align_structs()
    );

    Ok(())
}
