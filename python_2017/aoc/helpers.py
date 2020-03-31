from pathlib import Path

DATA_DIR = Path("/home/matt/Documents/AoC/python_2017/data")

def parse_input(input_file_dir: str):
    with open(input_file_dir, 'r') as f:
        out = f.readlines()
    return out
