import click

from aoc.helpers import DATA_DIR, parse_input
from aoc.registry import register

data = DATA_DIR / "day9.txt"


def remove_cancellations(s: str) -> str:
    ret = ""
    total_len = len(s)
    idx = 0
    while idx < total_len:
        if s[idx] != "!":
            ret = ret + s[idx]
            idx += 1
        else:
            idx += 2
    return ret


def remove_garbage(s: str) -> tuple[str, int]:
    ret = ""
    total_len = len(s)
    idx = 0
    garbage_count = 0
    while idx < total_len:
        if s[idx] != "<":
            ret = ret + s[idx]
            idx += 1
        else:
            new_idx = idx + 1
            while s[new_idx] != ">":
                new_idx += 1
                garbage_count += 1
            idx = new_idx + 1
    return ret, garbage_count


def remove_commas(s: str) -> str:
    ret = s.replace(",", "")
    return ret


def calc_score(s: str) -> int:
    total = 0
    tally = 0
    for char in s:
        if char == "{":
            tally += 1
        elif char == "}":
            total += tally
            tally -= 1
    return total


def part_one(s: str) -> int:
    no_cancels = remove_cancellations(s)
    no_garbage, _ = remove_garbage(no_cancels)
    no_comma = remove_commas(no_garbage)
    final_score = calc_score(no_comma)
    return final_score


def part_two(s: str) -> int:
    no_cancels = remove_cancellations(s)
    _, garbage_count = remove_garbage(no_cancels)
    return garbage_count


@register
@click.command()
@click.argument("part", type=int)
def day9(part: int) -> None:
    assert part in [
        1,
        2,
    ], f"The part number {part} is not implemented, please enter either 1 or 2."
    raw = parse_input(data)[0].strip()
    if part == 1:
        result = part_one(raw)
        print(f"The total score of all groups in the input is {result}")
    elif part == 2:
        result = part_two(raw)
        print(f"The total count of garbage characters is {result}")
