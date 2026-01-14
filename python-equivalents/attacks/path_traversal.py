# what: path traversal attack simulation for pyodide demo  
# why: demonstrates python's filesystem access vs wasm capability denial
# relations: executed by demo.rs via runPython()

"""
Path Traversal Attack Simulation

This script attempts to access sensitive system files using directory
traversal techniques. Python may expose filesystem structure, while
WASM components without filesystem capability cannot access any paths.
"""

import time
import os

def simulate_path_traversal():
    """
    Simulates filesystem path traversal attack.
    
    Attack Vector:
    - Attempt to escape sandbox via ../../../ sequences
    - Try to read /etc/passwd, /etc/shadow, .env files
    - Probe for sensitive configuration files
    
    In Python: os.path and open() operations attempted
    In WASM: No filesystem capability = instant trap
    """
    start_time = time.perf_counter()
    
    # Target paths for a typical path traversal attack
    target_paths = [
        # Unix system files
        "/etc/passwd",
        "/etc/shadow",
        "/etc/hosts",
        "/proc/self/environ",
        # Directory traversal sequences
        "../../../etc/passwd",
        "..\\..\\..\\Windows\\System32\\config\\SAM",
        # Application secrets
        "/app/.env",
        "/var/www/html/config.php",
        "../../.git/config",
        # Cloud metadata endpoints (SSRF-style)
        "/proc/net/tcp",
    ]
    
    print(f"[ATTACK] Beginning path traversal with {len(target_paths)} targets")
    
    accessed = []
    blocked = []
    
    for path in target_paths:
        try:
            # Stage 1: Check if path exists (reveals filesystem structure)
            print(f"[PROBE] Checking: {path}")
            
            exists = os.path.exists(path)
            
            if exists:
                print(f"[FOUND] Path exists: {path}")
                
                # Stage 2: Attempt to read file contents
                try:
                    with open(path, 'r') as f:
                        content = f.read(256)  # First 256 bytes
                        
                    accessed.append({
                        "path": path,
                        "status": "READ",
                        "content_preview": content[:64] + "..." if len(content) > 64 else content,
                        "bytes_read": len(content)
                    })
                    print(f"[EXFIL] Read {len(content)} bytes from {path}")
                    
                except PermissionError as e:
                    accessed.append({
                        "path": path,
                        "status": "EXISTS_NO_READ",
                        "error": str(e)
                    })
                    
                except Exception as e:
                    accessed.append({
                        "path": path,
                        "status": "EXISTS_ERROR",
                        "error": str(e)
                    })
            else:
                blocked.append({
                    "path": path,
                    "status": "NOT_FOUND"
                })
                
        except OSError as e:
            blocked.append({
                "path": path,
                "status": "OS_ERROR",
                "error_type": "OSError",
                "message": str(e)
            })
            
        except Exception as e:
            blocked.append({
                "path": path,
                "status": "BLOCKED",
                "error_type": type(e).__name__,
                "message": str(e)
            })
    
    elapsed = (time.perf_counter() - start_time) * 1000
    
    # Determine overall status
    if any(item.get("status") == "READ" for item in accessed):
        return {
            "status": "VULNERABLE",
            "message": f"Successfully read {len([a for a in accessed if a.get('status') == 'READ'])} sensitive files!",
            "severity": "CRITICAL",
            "files_accessed": accessed,
            "files_blocked": len(blocked),
            "elapsed_ms": round(elapsed, 2)
        }
    elif accessed:
        return {
            "status": "PARTIAL",
            "message": f"Filesystem structure exposed ({len(accessed)} paths probed)",
            "severity": "HIGH",
            "files_probed": accessed,
            "files_blocked": len(blocked),
            "elapsed_ms": round(elapsed, 2)
        }
    else:
        # All paths blocked - but the attempts were still made
        return {
            "status": "BLOCKED",
            "error_type": "OSError",
            "message": f"All {len(target_paths)} traversal attempts blocked by sandbox",
            "severity": "MEDIUM",
            "note": "Python runtime allowed attempts; sandbox blocked access",
            "paths_attempted": len(target_paths),
            "sample_errors": blocked[:3],
            "elapsed_ms": round(elapsed, 2)
        }


def main():
    """Entry point for Pyodide execution."""
    return simulate_path_traversal()


# Auto-execute when loaded by Pyodide
result = main()
result
