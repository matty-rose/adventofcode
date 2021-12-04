from pathlib import Path

DATA_DIR = Path(__file__).resolve().parent.parent / "data"


def parse_input(input_file_dir: str):
    with open(input_file_dir, "r") as f:
        out = f.readlines()
    return out
