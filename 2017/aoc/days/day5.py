import click

from aoc.helpers import DATA_DIR, parse_input
from aoc.registry import register

data_dir = DATA_DIR / "day5.txt"


def part_one(proc_inp: list[int]) -> int:
    final = len(proc_inp)
    cur_idx = 0
    steps = 0
    while True:
        if (cur_idx < 0) or (cur_idx >= final):
            break
        instruct = proc_inp[cur_idx]
        update = -1 if instruct >= 3 else 1
        proc_inp[cur_idx] = proc_inp[cur_idx] + update
        cur_idx = cur_idx + instruct
        steps += 1
    return steps


def part_two(proc_inp: list[int]) -> None:
    pass


@register
@click.command()
@click.argument("part", type=int)
def day5(part: int) -> None:
    assert part in [
        1,
    ], f"The part number {part} is not implemented, please enter either 1 or 2."
    raw_inp = parse_input(data_dir)
    int_inp = [int(p.strip()) for p in raw_inp]
    if part == 1:
        result = part_one(int_inp)
        print(f"The result for part 1 is {result}")
    else:
        raise NotImplementedError()
