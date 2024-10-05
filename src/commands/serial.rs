use std::time::Duration;

use serialport::SerialPort;

// pub(crate) is for testing
pub(crate) fn serial_session(device: &str, mut baud_rate: u32) -> Result<Box<dyn SerialPort>, Box<dyn std::error::Error>> {
    println!("Opening serial port: {}", device);
    
    if baud_rate == 0 {
        baud_rate = 9600;
    }
    
    let port = serialport::new(device, baud_rate)
        .timeout(Duration::from_millis(1000000))
        .open()?;
    
    // Testing
    println!("Serial port opened: {:?}", port);
    println!("Name: {:?}", port.name());

    Ok(port)
}


/* fn ssh_service(serial_connection: &mut Box<dyn SerialPort>) {
    // callt dann intern client.rs
} */
