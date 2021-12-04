import random

import click
from aoc.helpers import DATA_DIR, parse_input
from aoc.registry import register

data = DATA_DIR / "day12.txt"


def proc_raw(raw: list[str]) -> dict[int, list[int]]:
    ret = {}
    for line in raw:
        stripped = line.strip().split(" <-> ")
        prog = int(stripped[0])
        comms = [int(i) for i in stripped[1].split(", ")]
        ret[prog] = comms
    return ret


def get_connected(program: int, data: dict[int, list[int]], current_list: list[int]) -> list[int]:
    children = data[program]
    for c in children:
        if c in current_list:
            continue
        else:
            current_list.append(c)
            current_list = get_connected(c, data, current_list)
    return current_list


def part_one(proc: dict[int, list[int]], p: int = 0) -> int:
    all_progs = [p]
    all_connected = set(get_connected(p, proc, all_progs))
    return len(all_connected)


def part_two(proc: dict[int, list[int]]) -> int:
    num_groups = 0
    remaining_progs = set(proc.keys())
    while len(remaining_progs) != 0:
        el = random.choice(list(remaining_progs))
        el_group = set(get_connected(el, proc, [el]))
        remaining_progs = remaining_progs - el_group
        num_groups += 1
    return num_groups


@register
@click.command()
@click.argument("part", type=int)
def day12(part: int) -> None:
    assert part in [
        1,
        2,
    ], f"The part number {part} is not implemented, please enter either 1 or 2."
    raw = parse_input(data)
    proc = proc_raw(raw)
    if part == 1:
        result = part_one(proc)
        print(f"The solution for part 1 is {result}")
    elif part == 2:
        result = part_two(proc)
        print(f"The solution for part 2 is {result}")
