# Python Equivalents

These Python drivers demonstrate **industry-standard patterns** for ICS/SCADA sensor integration. They serve as the comparison baseline against WASM components in the demo.

## Files

| File | Purpose | Industry Patterns |
|------|---------|-------------------|
| `sensor_driver.py` | BME280 I2C sensor driver | Dataclasses, structured logging, retry logic, calibration handling |
| `modbus_parser.py` | Modbus RTU frame parser | CRC-16 validation, exception responses, lookup table optimization |

## Usage (via Pyodide)

```python
# In browser via Pyodide
import sensor_driver
result = sensor_driver.main()
print(result)

import modbus_parser
result = modbus_parser.main()
print(result)
```

## Industry Standards Demonstrated

- **IEC 61131-3**: Status enumerations (OK, WARNING, FAULT, OFFLINE)
- **Modbus RTU**: CRC-16 polynomial 0x8005, exception codes
- **Dataclass patterns**: Type-safe telemetry structures
- **Structured logging**: Machine-parseable log format
- **Retry with backoff**: Exponential backoff on sensor failures

## Size Comparison

| Component | Python | WASM |
|-----------|--------|------|
| Sensor driver | ~7 KB | ~3 KB |
| Modbus parser | ~9 KB | ~4 KB |
| **Total runtime** | **~12 MB** (Pyodide) | **~50 KB** |
