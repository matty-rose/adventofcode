from pathlib import Path
from typing import Any, Union

DATA_DIR = Path(__file__).resolve().parent.parent / "data"


def parse_input(input_file_dir: Union[str, bytes, Path]) -> list[Any]:
    with open(input_file_dir, "r") as f:
        out = f.readlines()
    return out
