import math
import os
from pathlib import Path

from aoc.helpers import DATA_DIR, parse_input

data = DATA_DIR / "day11.txt"

default_move_dict = {
    "n": [(0, 1), (0, 1)],
    "s": [(0, -1), (0, -1)],
    "ne": [(1, 0), (1, 1)],
    "nw": [(-1, 0), (-1, 1)],
    "se": [(1, -1), (1, 0)],
    "sw": [(-1, -1), (-1, 0)],
}


def get_steps(p):
    step_count = 0
    x, y = p
    if y == 0:
        step_count = abs(x)
    elif x == 0:
        step_count = abs(y)
    else:
        while x != 0:
            even_col = x % 2 == 0
            if (x > 0) and (y > 0):
                move = default_move_dict["sw"]
            elif (x > 0) and (y < 0):
                move = default_move_dict["nw"]
            elif (x < 0) and (y > 0):
                move = default_move_dict["se"]
            elif (x < 0) and (y < 0):
                move = default_move_dict["ne"]
            dx, dy = move[1] if even_col else move[0]
            x, y = x + dx, y + dy
            step_count += 1
        step_count += abs(y)
    return step_count


def part_one(steps):
    start_pos = (0, 0)
    x, y = start_pos
    for s in steps:
        even_col = x % 2 == 0
        ms = default_move_dict[s]
        dx, dy = ms[1] if even_col else ms[0]
        x, y = x + dx, y + dy
    steps = get_steps((x, y))
    return steps


def part_two(steps):
    start_pos = (0, 0)
    x, y = start_pos
    cur_max_steps = 0
    for s in steps:
        even_col = x % 2 == 0
        ms = default_move_dict[s]
        dx, dy = ms[1] if even_col else ms[0]
        x, y = x + dx, y + dy
        new_steps = get_steps((x, y))
        if new_steps > cur_max_steps:
            cur_max_steps = new_steps
    return cur_max_steps


if __name__ == "__main__":
    raw = parse_input(data)
    proc = raw[0].strip().split(",")
    print(part_one(proc))
    print(part_two(proc))
