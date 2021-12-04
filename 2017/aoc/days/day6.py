import click

from aoc.helpers import DATA_DIR, parse_input
from aoc.registry import register

data = DATA_DIR / "day6.txt"


def redistribute(max_idx: int, blocks: list[int]) -> list[int]:
    num_banks = len(blocks)
    num_blocks = blocks[max_idx]
    blocks[max_idx] = 0
    cur_idx = max_idx + 1
    while num_blocks != 0:
        blocks[cur_idx % num_banks] = blocks[cur_idx % num_banks] + 1
        cur_idx += 1
        num_blocks -= 1
    return blocks


def part_one(inp: list[int]) -> int:
    states = [inp]
    steps = 1
    cur_state = inp.copy()
    while True:
        max_idx = cur_state.index(max(cur_state))
        new_state = redistribute(max_idx, cur_state.copy())
        if new_state in states:
            break
        else:
            states.append(new_state)
            cur_state = new_state
            steps += 1
    return steps


def part_two(inp: list[int]) -> int:
    states = [inp]
    cur_state = inp.copy()
    while True:
        max_id = cur_state.index(max(cur_state))
        new_s = redistribute(max_id, cur_state.copy())
        if new_s in states:
            cycle_len = len(states) - states.index(new_s)
            return cycle_len
        else:
            states.append(new_s)
            cur_state = new_s


@register
@click.command()
@click.argument("part", type=int)
def day6(part: int) -> None:
    assert part in [
        1,
        2,
    ], f"The part number {part} is not implemented, please enter either 1 or 2."
    raw_inp = parse_input(data)
    proc_inp = [int(i) for i in raw_inp[0].strip().split("\t")]
    if part == 1:
        result = part_one(proc_inp)
        print(f"The solution for part 1 is {result}")
    elif part == 2:
        result = part_two(proc_inp)
        print(f"The solution for part 2 is {result}")
