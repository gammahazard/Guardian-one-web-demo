// what: sensor driver logic for reading bme280 telemetry
// why: demonstrates wasi component model for industrial sensors
// relations: compiled to .wasm, called from dashboard for comparison

use wasm_bindgen::prelude::*;

/// Sensor telemetry data
#[wasm_bindgen]
pub struct Telemetry {
    temperature: f32,
    humidity: f32,
    pressure: f32,
}

#[wasm_bindgen]
impl Telemetry {
    #[wasm_bindgen(getter)]
    pub fn temperature(&self) -> f32 {
        self.temperature
    }

    #[wasm_bindgen(getter)]
    pub fn humidity(&self) -> f32 {
        self.humidity
    }

    #[wasm_bindgen(getter)]
    pub fn pressure(&self) -> f32 {
        self.pressure
    }
}

/// Initialize sensor (simulates I2C init and calibration load)
#[wasm_bindgen]
pub fn init_sensor() -> bool {
    // Simulates:
    // 1. I2C bus open
    // 2. Chip ID verification
    // 3. Calibration data load
    // 4. Sensor configuration
    true
}

/// Read sensor data
/// In real implementation: reads I2C registers, applies calibration
#[wasm_bindgen]
pub fn read_sensor() -> Telemetry {
    // Simulated values (same as Python for fair comparison)
    Telemetry {
        temperature: 23.5,
        humidity: 45.2,
        pressure: 1013.25,
    }
}

/// Main entry point - initialize and read
#[wasm_bindgen]
pub fn sensor_check() -> Result<Telemetry, JsValue> {
    if !init_sensor() {
        return Err(JsValue::from_str("Sensor initialization failed"));
    }
    Ok(read_sensor())
}
