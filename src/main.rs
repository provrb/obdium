use obdium::cmd::Command;
use obdium::obd::OBD;

fn main() -> std::io::Result<()> {
    let mut obd = OBD::new();
    if !obd.connect("127.0.0.1", "5054") {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Couldn't connect with TCP",
        ));
    }

    if !obd.send_request(Command::new(b"010C")) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to send request",
        ));
    }

    let response = match obd.get_response() {
        Some(res) => res,
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to receive response",
            ))
        }
    };
    
    println!("Payload: {}", response.get_payload().unwrap());
    response.get_payload_bytes();

    Ok(())
}
