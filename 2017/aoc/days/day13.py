import itertools

import click

from aoc.helpers import DATA_DIR, parse_input
from aoc.registry import register

data = DATA_DIR / "day13.txt"


def scanner(height: int, time: int) -> bool:
    offsets = list(range(height)) + list(range(height - 1, 0, -1))[1:]
    current_offset = offsets[time % len(offsets)]
    return True if current_offset == 0 else False


def part_one(proc: dict[int, int]) -> int:
    severity = sum([time * proc[time] for time in proc if scanner(proc[time], time)])
    return severity


def part_two(proc: dict[int, int]) -> int:
    return next(
        delay for delay in itertools.count() if not any(scanner(proc[t], t + delay) for t in proc)
    )


@register
@click.command()
@click.argument("part", type=int)
def day13(part: int) -> None:
    assert part in [
        1,
        2,
    ], f"The part number {part} is not implemented, please enter either 1 or 2."

    raw = parse_input(data)
    proc: dict[int, int] = {}
    for line in raw:
        n = list(map(int, line.strip().split(": ")))
        proc[n[0]] = n[1]

    if part == 1:
        result = part_one(proc)
        print(f"The solution for part 1 is {result}")
    elif part == 2:
        result = part_two(proc)
        print(f"The solution for part 2 is {result}")
