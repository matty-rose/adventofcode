import itertools
from pathlib import Path
from typing import Dict

from aoc.helpers import DATA_DIR, parse_input

data: Path = DATA_DIR / "day13.txt"


def scanner(height: int, time: int) -> bool:
    offsets = list(range(height)) + list(range(height - 1, 0, -1))[1:]
    current_offset = offsets[time % len(offsets)]
    return True if current_offset == 0 else False


def part_one(proc: Dict[int, int]) -> int:
    severity = sum([time * proc[time] for time in proc if scanner(proc[time], time)])
    return severity


def part_two(proc: Dict[int, int]) -> int:
    return next(
        delay
        for delay in itertools.count()
        if not any(scanner(proc[t], t + delay) for t in proc)
    )


if __name__ == "__main__":
    raw = parse_input(data)
    proc: Dict[int, int] = {}
    for line in raw:
        n = list(map(int, line.strip().split(": ")))
        proc[n[0]] = n[1]

    print(part_one(proc))
    print(part_two(proc))
