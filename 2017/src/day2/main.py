import itertools as it
import os
from pathlib import Path

from aoc.helpers import DATA_DIR, parse_input


def get_2d_input():
    inp_path = DATA_DIR / "day2.txt"
    raw_output = parse_input(str(inp_path))
    output = [list(map(int, line.strip().split("\t"))) for line in raw_output]
    return output


def part_one():
    data = get_2d_input()
    diffs = map(lambda x: max(x) - min(x), data)
    checksum = sum(diffs)
    return checksum


def part_two():
    data = get_2d_input()
    perms = map(lambda x: it.permutations(x, 2), data)
    greater = [list(filter(lambda x: x[0] % x[1] == 0, p)) for p in perms]
    divs = map(lambda x: x[0][0] / x[0][1], greater)
    checksum = sum(divs)
    return checksum


if __name__ == "__main__":
    result = part_one()
    print(f"The checksum for part 1 is {result}")

    result_two = part_two()
    print(f"The checksum for part 2 is {result_two}")
