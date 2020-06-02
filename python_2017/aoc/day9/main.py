import os
from pathlib import Path

from aoc.helpers import DATA_DIR, parse_input

data = DATA_DIR / "day9.txt"


def remove_cancellations(s):
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


def remove_garbage(s):
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


def remove_commas(s):
    ret = s.replace(",", "")
    return ret


def calc_score(s):
    total = 0
    tally = 0
    for char in s:
        if char == "{":
            tally += 1
        elif char == "}":
            total += tally
            tally -= 1
    return total


def part_one(s):
    no_cancels = remove_cancellations(s)
    no_garbage, _ = remove_garbage(no_cancels)
    no_comma = remove_commas(no_garbage)
    final_score = calc_score(no_comma)
    return final_score


def part_two(s):
    no_cancels = remove_cancellations(s)
    _, garbage_count = remove_garbage(no_cancels)
    return garbage_count


if __name__ == "__main__":
    raw = parse_input(data)
    raw = raw[0].strip()
    print(f"The total score of all groups in the input is {part_one(raw)}")
    print(f"The total count of garbage characters is {part_two(raw)}")
