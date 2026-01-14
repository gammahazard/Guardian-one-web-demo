// what: sensor driver logic for reading bme280 telemetry
// why: demonstrates wasi component model for industrial sensors
// relations: compiled to .wasm, used by dashboard for comparison

/// Sensor telemetry data
pub struct Telemetry {
    pub temperature: f32,
    pub humidity: f32,
    pub pressure: f32,
}

/// Read sensor data (simulated for web demo)
pub fn read_sensor() -> Result<Telemetry, &'static str> {
    Ok(Telemetry {
        temperature: 23.5,
        humidity: 45.2,
        pressure: 1013.25,
    })
}
