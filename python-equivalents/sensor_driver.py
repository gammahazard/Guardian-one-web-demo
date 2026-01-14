"""
what: bme280 sensor driver with industry-standard patterns
why: demonstrates typical python ics driver complexity for comparison
relations: loaded by pyodide in browser, compared against wasm version
"""

from dataclasses import dataclass
from datetime import datetime
from enum import Enum
from typing import Optional
import logging
import struct
import time

# Configure structured logging (industry standard)
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)


class SensorStatus(Enum):
    """Sensor health states per IEC 61131-3 conventions."""
    OK = 0
    WARNING = 1
    FAULT = 2
    OFFLINE = 3


@dataclass
class CalibrationData:
    """BME280 calibration coefficients (from datasheet)."""
    dig_T1: int
    dig_T2: int
    dig_T3: int
    dig_P1: int
    dig_P2: int
    dig_P3: int
    dig_H1: int
    dig_H2: int
    # ... additional coefficients omitted for brevity


@dataclass
class Telemetry:
    """Sensor telemetry with metadata."""
    temperature: float
    humidity: float
    pressure: float
    timestamp: datetime
    status: SensorStatus
    sequence_number: int
    
    def to_dict(self) -> dict:
        """Serialize for JSON/MQTT transport."""
        return {
            "temperature_c": round(self.temperature, 2),
            "humidity_pct": round(self.humidity, 2),
            "pressure_hpa": round(self.pressure, 2),
            "timestamp_iso": self.timestamp.isoformat(),
            "status": self.status.name,
            "seq": self.sequence_number,
        }


class BME280Driver:
    """
    Industry-standard BME280 I2C sensor driver.
    
    Implements:
    - Retry logic with exponential backoff
    - Calibration coefficient handling
    - CRC validation (when applicable)
    - Structured error handling
    - Sequence numbering for telemetry integrity
    """
    
    I2C_ADDRESS = 0x76
    CHIP_ID_REGISTER = 0xD0
    EXPECTED_CHIP_ID = 0x60
    
    # Timing constants (from datasheet)
    MEASUREMENT_TIME_MS = 40
    STARTUP_TIME_MS = 2
    
    def __init__(self, i2c_bus: int = 1, address: int = 0x76):
        self.i2c_bus = i2c_bus
        self.address = address
        self._sequence = 0
        self._calibration: Optional[CalibrationData] = None
        self._last_reading: Optional[Telemetry] = None
        self._consecutive_failures = 0
        self._max_retries = 3
        
        logger.info(f"BME280 driver initialized on bus {i2c_bus}, address 0x{address:02X}")
    
    def initialize(self) -> bool:
        """
        Initialize sensor and load calibration data.
        
        Returns:
            True if initialization successful, False otherwise.
        """
        try:
            # Verify chip ID (simulated for browser demo)
            chip_id = self._read_register(self.CHIP_ID_REGISTER)
            if chip_id != self.EXPECTED_CHIP_ID:
                logger.error(f"Unexpected chip ID: 0x{chip_id:02X}")
                return False
            
            # Load calibration coefficients
            self._load_calibration()
            
            # Configure for forced mode, 1x oversampling
            self._configure_sensor()
            
            logger.info("BME280 initialization complete")
            return True
            
        except Exception as e:
            logger.exception(f"Initialization failed: {e}")
            return False
    
    def read(self) -> Telemetry:
        """
        Read sensor with retry logic and validation.
        
        Implements exponential backoff on failure.
        """
        for attempt in range(self._max_retries):
            try:
                # Trigger measurement
                self._trigger_measurement()
                
                # Wait for conversion (industry practice: poll status register)
                time.sleep(self.MEASUREMENT_TIME_MS / 1000)
                
                # Read raw data
                raw_data = self._read_raw_data()
                
                # Apply calibration compensation
                temp, hum, pres = self._compensate(raw_data)
                
                # Validate readings are within physical bounds
                if not self._validate_readings(temp, hum, pres):
                    raise ValueError("Readings outside valid range")
                
                self._sequence += 1
                self._consecutive_failures = 0
                
                telemetry = Telemetry(
                    temperature=temp,
                    humidity=hum,
                    pressure=pres,
                    timestamp=datetime.utcnow(),
                    status=SensorStatus.OK,
                    sequence_number=self._sequence,
                )
                
                self._last_reading = telemetry
                logger.debug(f"Read successful: {temp:.1f}°C, {hum:.1f}%, {pres:.1f}hPa")
                return telemetry
                
            except Exception as e:
                self._consecutive_failures += 1
                wait_time = (2 ** attempt) * 0.1  # Exponential backoff
                logger.warning(f"Read attempt {attempt + 1} failed: {e}, waiting {wait_time}s")
                time.sleep(wait_time)
        
        # All retries exhausted
        logger.error(f"Sensor read failed after {self._max_retries} attempts")
        return Telemetry(
            temperature=0.0,
            humidity=0.0,
            pressure=0.0,
            timestamp=datetime.utcnow(),
            status=SensorStatus.FAULT,
            sequence_number=self._sequence,
        )
    
    def _read_register(self, register: int) -> int:
        """Read single register (simulated for browser)."""
        # In real driver: return smbus.read_byte_data(self.address, register)
        if register == self.CHIP_ID_REGISTER:
            return self.EXPECTED_CHIP_ID
        return 0
    
    def _load_calibration(self) -> None:
        """Load factory calibration from sensor EEPROM."""
        # Simulated calibration data
        self._calibration = CalibrationData(
            dig_T1=27504, dig_T2=26435, dig_T3=-1000,
            dig_P1=36477, dig_P2=-10685, dig_P3=3024,
            dig_H1=75, dig_H2=370,
        )
        logger.debug("Calibration data loaded")
    
    def _configure_sensor(self) -> None:
        """Configure sensor operating mode."""
        # ctrl_hum: humidity oversampling x1
        # ctrl_meas: temp/pressure oversampling x1, forced mode
        logger.debug("Sensor configured for forced mode")
    
    def _trigger_measurement(self) -> None:
        """Trigger single measurement in forced mode."""
        pass  # Simulated
    
    def _read_raw_data(self) -> bytes:
        """Read raw ADC data from sensor."""
        # Return simulated raw data representing typical room conditions
        return struct.pack(">I", 0x50000) + struct.pack(">I", 0x80000) + struct.pack(">H", 0x8000)
    
    def _compensate(self, raw_data: bytes) -> tuple[float, float, float]:
        """Apply calibration compensation algorithms."""
        # Simulated realistic values
        return (23.5, 45.2, 1013.25)
    
    def _validate_readings(self, temp: float, hum: float, pres: float) -> bool:
        """Validate readings are within physical bounds."""
        return (
            -40.0 <= temp <= 85.0 and
            0.0 <= hum <= 100.0 and
            300.0 <= pres <= 1100.0
        )


# Entry point for Pyodide execution
def main() -> dict:
    """Main entry point for browser demo."""
    start_time = time.perf_counter()
    
    driver = BME280Driver()
    if not driver.initialize():
        return {"error": "Initialization failed"}
    
    reading = driver.read()
    
    elapsed_ms = (time.perf_counter() - start_time) * 1000
    
    result = reading.to_dict()
    result["execution_time_ms"] = round(elapsed_ms, 2)
    result["driver_size_bytes"] = len(open(__file__).read().encode())
    
    return result


if __name__ == "__main__":
    print(main())
