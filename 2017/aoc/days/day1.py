import click

from aoc.helpers import DATA_DIR, parse_input
from aoc.registry import register


def get_digit_list() -> list[int]:
    seq = parse_input(str(DATA_DIR / "day1.txt"))
    number_str = seq[0].strip()
    num_list = list(map(int, number_str))
    return num_list


def part_one(num_list: list[int]) -> int:
    total_length = len(num_list)
    ret = 0
    for i, _ in enumerate(num_list):
        if i < total_length - 1:
            if num_list[i] == num_list[i + 1]:
                ret += num_list[i]
        else:
            if num_list[i] == num_list[0]:
                ret += num_list[i]
    return ret


def part_two(num_list: list[int]) -> int:
    total_length = len(num_list)
    half = total_length // 2
    ret = 0
    for i, _ in enumerate(num_list):
        new_pos = (i + half) % total_length
        if num_list[i] == num_list[new_pos]:
            ret += num_list[i]
    return ret


@register
@click.command()
@click.argument("part", type=int)
def day1(part: int) -> None:
    assert part in [
        1,
        2,
    ], f"The part number {part} is not implemented, please enter either 1 or 2."
    digit_list = get_digit_list()
    if part == 1:
        final_sum = part_one(digit_list)
    else:
        final_sum = part_two(digit_list)
    print(f"The total sum for part {part} is {final_sum}.")
