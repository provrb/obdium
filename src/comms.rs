struct PID {
    pid_hex: [u8; 2], 
    service_num: u8,
    command: [u8; 2], // service_num + pid_hex
    response: Option<String>, // Hex Response from ECU
    bytes: u8, // How many bytes in the response
}

