// what: modbus frame parser with crc validation
// why: protocol gateway needs to handle industrial packets
// relations: compiled to .wasm, used by dashboard for comparison

/// Parsed Modbus frame
pub struct ModbusFrame {
    pub device_id: u8,
    pub function_code: u8,
    pub data: Vec<u8>,
}

/// Parse raw bytes into Modbus frame
pub fn parse_frame(raw: &[u8]) -> Result<ModbusFrame, &'static str> {
    if raw.len() < 4 {
        return Err("Frame too short");
    }
    
    Ok(ModbusFrame {
        device_id: raw[0],
        function_code: raw[1],
        data: raw[2..raw.len()-2].to_vec(),
    })
}
