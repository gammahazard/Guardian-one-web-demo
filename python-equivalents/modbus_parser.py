"""
what: modbus rtu frame parser with crc-16 validation
why: demonstrates industrial protocol parsing complexity for comparison
relations: loaded by pyodide in browser, compared against wasm version
"""

from dataclasses import dataclass
from enum import IntEnum
from typing import Optional
import logging
import struct
import time

logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)


class FunctionCode(IntEnum):
    """Modbus function codes per specification."""
    READ_COILS = 0x01
    READ_DISCRETE_INPUTS = 0x02
    READ_HOLDING_REGISTERS = 0x03
    READ_INPUT_REGISTERS = 0x04
    WRITE_SINGLE_COIL = 0x05
    WRITE_SINGLE_REGISTER = 0x06
    WRITE_MULTIPLE_COILS = 0x0F
    WRITE_MULTIPLE_REGISTERS = 0x10
    
    # Exception codes (function code + 0x80)
    EXCEPTION_OFFSET = 0x80


class ExceptionCode(IntEnum):
    """Modbus exception codes."""
    ILLEGAL_FUNCTION = 0x01
    ILLEGAL_DATA_ADDRESS = 0x02
    ILLEGAL_DATA_VALUE = 0x03
    SLAVE_DEVICE_FAILURE = 0x04
    ACKNOWLEDGE = 0x05
    SLAVE_DEVICE_BUSY = 0x06


@dataclass
class ModbusFrame:
    """Parsed Modbus RTU frame."""
    device_id: int
    function_code: int
    data: bytes
    crc_valid: bool
    raw_frame: bytes
    
    @property
    def is_exception(self) -> bool:
        """Check if this is an exception response."""
        return self.function_code >= FunctionCode.EXCEPTION_OFFSET
    
    @property
    def exception_code(self) -> Optional[ExceptionCode]:
        """Get exception code if this is an exception response."""
        if self.is_exception and len(self.data) >= 1:
            return ExceptionCode(self.data[0])
        return None
    
    def to_dict(self) -> dict:
        """Serialize for JSON transport."""
        return {
            "device_id": self.device_id,
            "function_code": self.function_code,
            "function_name": self._get_function_name(),
            "data_hex": self.data.hex(),
            "data_length": len(self.data),
            "crc_valid": self.crc_valid,
            "is_exception": self.is_exception,
        }
    
    def _get_function_name(self) -> str:
        """Get human-readable function name."""
        try:
            if self.is_exception:
                base_code = self.function_code - FunctionCode.EXCEPTION_OFFSET
                return f"EXCEPTION_{FunctionCode(base_code).name}"
            return FunctionCode(self.function_code).name
        except ValueError:
            return f"UNKNOWN_0x{self.function_code:02X}"


class CRC16Modbus:
    """
    Modbus CRC-16 calculator.
    
    Polynomial: 0x8005 (reflected: 0xA001)
    Initial value: 0xFFFF
    """
    
    # Pre-computed lookup table for performance
    _TABLE = None
    
    @classmethod
    def _generate_table(cls) -> list[int]:
        """Generate CRC lookup table."""
        table = []
        for i in range(256):
            crc = i
            for _ in range(8):
                if crc & 1:
                    crc = (crc >> 1) ^ 0xA001
                else:
                    crc >>= 1
            table.append(crc)
        return table
    
    @classmethod
    def calculate(cls, data: bytes) -> int:
        """Calculate CRC-16 for Modbus RTU frame."""
        if cls._TABLE is None:
            cls._TABLE = cls._generate_table()
        
        crc = 0xFFFF
        for byte in data:
            crc = (crc >> 8) ^ cls._TABLE[(crc ^ byte) & 0xFF]
        
        return crc
    
    @classmethod
    def verify(cls, frame: bytes) -> bool:
        """Verify CRC of complete frame (data + CRC)."""
        if len(frame) < 4:
            return False
        
        data = frame[:-2]
        received_crc = struct.unpack("<H", frame[-2:])[0]
        calculated_crc = cls.calculate(data)
        
        return received_crc == calculated_crc


class ModbusParser:
    """
    Industrial Modbus RTU frame parser.
    
    Implements:
    - CRC-16 validation
    - Exception response handling
    - Strict frame length validation
    - Structured error reporting
    """
    
    MIN_FRAME_LENGTH = 4  # device_id(1) + function(1) + crc(2)
    MAX_FRAME_LENGTH = 256  # Modbus RTU max ADU size
    
    def __init__(self, strict_mode: bool = True):
        """
        Initialize parser.
        
        Args:
            strict_mode: If True, reject frames with invalid CRC.
        """
        self.strict_mode = strict_mode
        self._frames_parsed = 0
        self._crc_failures = 0
        
        logger.info(f"ModbusParser initialized (strict_mode={strict_mode})")
    
    def parse(self, raw: bytes) -> ModbusFrame:
        """
        Parse raw bytes into Modbus frame.
        
        Args:
            raw: Raw frame bytes including CRC.
            
        Returns:
            Parsed ModbusFrame object.
            
        Raises:
            ValueError: If frame is malformed.
        """
        # Length validation
        if len(raw) < self.MIN_FRAME_LENGTH:
            raise ValueError(f"Frame too short: {len(raw)} bytes (min: {self.MIN_FRAME_LENGTH})")
        
        if len(raw) > self.MAX_FRAME_LENGTH:
            raise ValueError(f"Frame too long: {len(raw)} bytes (max: {self.MAX_FRAME_LENGTH})")
        
        # CRC validation
        crc_valid = CRC16Modbus.verify(raw)
        if not crc_valid:
            self._crc_failures += 1
            logger.warning(f"CRC validation failed (total failures: {self._crc_failures})")
            if self.strict_mode:
                raise ValueError("CRC validation failed")
        
        # Parse header
        device_id = raw[0]
        function_code = raw[1]
        
        # Extract data (excluding device_id, function_code, and 2-byte CRC)
        data = raw[2:-2] if len(raw) > 4 else b""
        
        self._frames_parsed += 1
        
        frame = ModbusFrame(
            device_id=device_id,
            function_code=function_code,
            data=data,
            crc_valid=crc_valid,
            raw_frame=raw,
        )
        
        logger.debug(f"Parsed frame: {frame.to_dict()}")
        return frame
    
    def build_request(
        self,
        device_id: int,
        function_code: FunctionCode,
        start_address: int,
        quantity: int,
    ) -> bytes:
        """
        Build a Modbus request frame.
        
        Args:
            device_id: Slave device address (1-247).
            function_code: Modbus function code.
            start_address: Starting register/coil address.
            quantity: Number of registers/coils to read.
            
        Returns:
            Complete frame with CRC.
        """
        # Validate inputs
        if not 1 <= device_id <= 247:
            raise ValueError(f"Invalid device ID: {device_id}")
        
        if not 1 <= quantity <= 125:
            raise ValueError(f"Invalid quantity: {quantity}")
        
        # Build frame without CRC
        frame = struct.pack(
            ">BBHH",
            device_id,
            function_code,
            start_address,
            quantity,
        )
        
        # Append CRC (little-endian)
        crc = CRC16Modbus.calculate(frame)
        frame += struct.pack("<H", crc)
        
        logger.debug(f"Built request: {frame.hex()}")
        return frame
    
    @property
    def stats(self) -> dict:
        """Get parser statistics."""
        return {
            "frames_parsed": self._frames_parsed,
            "crc_failures": self._crc_failures,
            "failure_rate": self._crc_failures / max(self._frames_parsed, 1),
        }


# Entry point for Pyodide execution
def main() -> dict:
    """Main entry point for browser demo."""
    start_time = time.perf_counter()
    
    parser = ModbusParser(strict_mode=False)
    
    # Test cases
    results = []
    
    # Valid frame: Read holding registers from device 1
    valid_frame = parser.build_request(
        device_id=1,
        function_code=FunctionCode.READ_HOLDING_REGISTERS,
        start_address=0x0000,
        quantity=10,
    )
    
    try:
        parsed = parser.parse(valid_frame)
        results.append({"test": "valid_frame", "success": True, **parsed.to_dict()})
    except ValueError as e:
        results.append({"test": "valid_frame", "success": False, "error": str(e)})
    
    # Invalid frame: Too short
    try:
        parser.parse(b"\x01\x03")
        results.append({"test": "too_short", "success": False, "error": "Should have raised"})
    except ValueError as e:
        results.append({"test": "too_short", "success": True, "caught_error": str(e)})
    
    # Invalid frame: Bad CRC
    try:
        parser.parse(b"\x01\x03\x00\x00\x00\x0A\xFF\xFF")
        results.append({"test": "bad_crc", "success": True, "note": "Accepted with warning"})
    except ValueError as e:
        results.append({"test": "bad_crc", "success": True, "caught_error": str(e)})
    
    elapsed_ms = (time.perf_counter() - start_time) * 1000
    
    return {
        "results": results,
        "stats": parser.stats,
        "execution_time_ms": round(elapsed_ms, 2),
    }


if __name__ == "__main__":
    import json
    print(json.dumps(main(), indent=2))
