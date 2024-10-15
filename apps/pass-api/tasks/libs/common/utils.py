"""
Miscellaneous functions, no tasks here
"""

from __future__ import annotations

import time
from contextlib import contextmanager
from types import SimpleNamespace

@contextmanager
def timed(name="", quiet=False):
    """Context manager that prints how long it took"""
    start = time.time()
    res = SimpleNamespace()
    print(f"{name}")
    try:
        yield res
    finally:
        res.duration = time.time() - start
        if not quiet:
            print(f"{name} completed in {res.duration:.2f}s")

