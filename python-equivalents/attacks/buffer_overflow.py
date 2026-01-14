# what: buffer overflow attack simulation for pyodide demo
# why: demonstrates python's memory handling vs wasm's bounded linear memory
# relations: executed by demo.rs via runPython()

"""
Buffer Overflow Attack Simulation

This script attempts operations that would cause memory corruption in
traditional runtimes. Python raises MemoryError or IndexError, while
WASM traps safely with "out of bounds memory access".
"""

import sys
import time

def simulate_buffer_overflow():
    """
    Simulates memory corruption attack.
    
    Attack Vector:
    - Attempt to allocate massive buffer (triggers MemoryError)
    - Attempt out-of-bounds array access (triggers IndexError)
    
    In Python: Raises exception, process may crash
    In WASM: Linear memory bounds-checked, traps safely
    """
    start_time = time.perf_counter()
    
    try:
        # Stage 1: Attempt massive allocation
        # This simulates a heap spray attack
        print("[ATTACK] Attempting heap spray (256MB allocation)...")
        
        try:
            # 256MB allocation - will fail in constrained environments
            massive_buffer = bytearray(256 * 1024 * 1024)
            print(f"[WARN] Allocated {len(massive_buffer)} bytes - heap spray succeeded!")
        except MemoryError:
            print("[INFO] MemoryError on heap spray - continuing to stage 2...")
        
        # Stage 2: Stack-based buffer overflow simulation
        print("[ATTACK] Attempting stack buffer overflow...")
        
        # Create a fixed-size buffer
        fixed_buffer = bytearray(64)
        
        # Attempt to write beyond bounds (classic buffer overflow)
        overflow_data = b"A" * 128  # Twice the buffer size
        
        for i, byte in enumerate(overflow_data):
            # This will raise IndexError when i >= 64
            fixed_buffer[i] = byte
        
        # If we get here, something is very wrong
        elapsed = (time.perf_counter() - start_time) * 1000
        return {
            "status": "VULNERABLE",
            "message": "Buffer overflow succeeded - memory corrupted!",
            "severity": "CRITICAL",
            "elapsed_ms": round(elapsed, 2)
        }
        
    except MemoryError as e:
        elapsed = (time.perf_counter() - start_time) * 1000
        return {
            "status": "CRASHED",
            "error_type": "MemoryError",
            "message": str(e) if str(e) else "Unable to allocate memory",
            "severity": "HIGH",
            "elapsed_ms": round(elapsed, 2),
            "traceback": f"MemoryError at heap allocation"
        }
        
    except IndexError as e:
        elapsed = (time.perf_counter() - start_time) * 1000
        return {
            "status": "CRASHED",
            "error_type": "IndexError", 
            "message": str(e),
            "severity": "HIGH",
            "elapsed_ms": round(elapsed, 2),
            "traceback": f"IndexError: buffer[{len(fixed_buffer)}] is out of bounds"
        }
        
    except Exception as e:
        elapsed = (time.perf_counter() - start_time) * 1000
        return {
            "status": "CRASHED",
            "error_type": type(e).__name__,
            "message": str(e),
            "severity": "MEDIUM",
            "elapsed_ms": round(elapsed, 2)
        }


def main():
    """Entry point for Pyodide execution."""
    return simulate_buffer_overflow()


# Auto-execute when loaded by Pyodide
result = main()
result
