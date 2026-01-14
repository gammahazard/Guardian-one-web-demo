# what: attack simulation module for pyodide demo
# why: provides real python attack code that produces genuine exceptions
# relations: loaded by demo.rs via runPython()

"""
Attack Simulation Module

This module contains Python attack scripts that demonstrate real error handling
differences between Python and WASM. Each attack produces genuine exceptions
that are captured and displayed in the Reliability Triad Console.
"""

from . import buffer_overflow
from . import data_exfil
from . import path_traversal
