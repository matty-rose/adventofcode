import itertools as it

import click
from aoc.helpers import DATA_DIR, parse_input
from aoc.registry import register


def get_2d_input() -> list[list[int]]:
    inp_path = DATA_DIR / "day2.txt"
    raw_output = parse_input(str(inp_path))
    output = [list(map(int, line.strip().split("\t"))) for line in raw_output]
    return output


def part_one() -> float:
    data = get_2d_input()
    diffs = map(lambda x: max(x) - min(x), data)
    checksum = sum(diffs)
    return checksum


def part_two() -> float:
    data = get_2d_input()
    perms = map(lambda x: it.permutations(x, 2), data)
    greater = [list(filter(lambda x: x[0] % x[1] == 0, p)) for p in perms]
    divs = map(lambda x: x[0][0] / x[0][1], greater)
    checksum = sum(divs)
    return checksum


@register
@click.command()
@click.argument("part", type=int)
def day2(part: int) -> None:
    assert part in [
        1,
        2,
    ], f"The part number {part} is not implemented, please enter either 1 or 2."
    if part == 1:
        result = part_one()
        print(f"The checksum for part 1 is {result}")
    elif part == 2:
        result = part_two()
        print(f"The checksum for part 2 is {result}")
