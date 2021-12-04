import click

from aoc.helpers import DATA_DIR, parse_input
from aoc.registry import register

data = DATA_DIR / "day11.txt"

Point = tuple[int, int]

default_move_dict: dict[str, list[Point]] = {
    "n": [(0, 1), (0, 1)],
    "s": [(0, -1), (0, -1)],
    "ne": [(1, 0), (1, 1)],
    "nw": [(-1, 0), (-1, 1)],
    "se": [(1, -1), (1, 0)],
    "sw": [(-1, -1), (-1, 0)],
}


def get_steps(p: Point) -> int:
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


def part_one(steps: list[str]) -> int:
    start_pos = (0, 0)
    x, y = start_pos
    for s in steps:
        even_col = x % 2 == 0
        ms = default_move_dict[s]
        dx, dy = ms[1] if even_col else ms[0]
        x, y = x + dx, y + dy
    step_count = get_steps((x, y))
    return step_count


def part_two(steps: list[str]) -> int:
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


@register
@click.command()
@click.argument("part", type=int)
def day11(part: int) -> None:
    assert part in [
        1,
        2,
    ], f"The part number {part} is not implemented, please enter either 1 or 2."
    raw = parse_input(data)
    proc = raw[0].strip().split(",")
    if part == 1:
        result = part_one(proc)
        print(f"The solution for part 1 is {result}")
    elif part == 2:
        result = part_two(proc)
        print(f"The solution for part 2 is {result}")
