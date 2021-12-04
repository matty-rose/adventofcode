import logging
from importlib import import_module
from pathlib import Path

days = [x.stem for x in Path(__file__).parent.glob("**/day*.py")]

for day in days:
    module_name = f"aoc.days.{day}"
    try:
        import_module(module_name)
    except Exception:
        logging.warning(f"couldn't import {module_name}")
