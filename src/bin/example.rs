use std::time::Duration;

use serialport::SerialPortType;

use znp::ZNP;
use znp_types::command::de::Command;
use znp_types::command::sys::Ping;

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
    let mut controller = ZNP::from_port(port)?;

    println!("pinging...");
    let command = Ping {};
    controller.send_command(command.clone())?;

    loop {
        let mut exec_fn = || -> Result<_, Box<dyn std::error::Error>> {
            let frame = controller.recv_frame()?;
            let capabilities = command.deserialize(frame.command)?;
            Ok(capabilities)
        };
        if let Ok(capabilities) = exec_fn() {
            println!("capabilities: {capabilities}");
            break;
        }
        std::thread::sleep(Duration::from_millis(500));
    }

    Ok(())
}
