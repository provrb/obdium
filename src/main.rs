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

    println!("RPM: {}", obd.rpm());

    Ok(())
}
