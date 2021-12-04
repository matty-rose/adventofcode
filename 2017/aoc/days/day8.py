from typing import Any

import click

from aoc.helpers import DATA_DIR, parse_input
from aoc.registry import register

dpath = DATA_DIR / "day8.txt"


def proc_line(line: str) -> dict[str, Any]:
    line = line.strip()
    split = line.split(" ")
    ret = {
        "register": split[0],
        "op": split[1],
        "val": int(split[2]),
        "cond": " ".join(split[3:]),
    }
    return ret


def part_one(instructions: list[dict[str, Any]]) -> int:
    registers = {r: 0 for r in list(set([ins["register"] for ins in instructions]))}
    for ins in instructions:
        reg = ins["register"]
        split_cond = ins["cond"].split()
        split_cond[1] = f"registers['{split_cond[1]}']"
        altered_cond = " ".join(split_cond[1:])
        if eval(altered_cond):
            if ins["op"] == "inc":
                registers[reg] = registers[reg] + ins["val"]
            elif ins["op"] == "dec":
                registers[reg] = registers[reg] - ins["val"]
            else:
                print("This operation isn't valid")
                continue
    max_val = max(registers.values())
    return max_val


def part_two(instructions: list[dict[str, Any]]) -> int:
    registers = {r: 0 for r in list(set([ins["register"] for ins in instructions]))}
    cur_max = 0
    for ins in instructions:
        reg = ins["register"]
        split_cond = ins["cond"].split()
        split_cond[1] = f"registers['{split_cond[1]}']"
        altered_cond = " ".join(split_cond[1:])
        if eval(altered_cond):
            if ins["op"] == "inc":
                registers[reg] = registers[reg] + ins["val"]
                if registers[reg] > cur_max:
                    cur_max = registers[reg]
            elif ins["op"] == "dec":
                registers[reg] = registers[reg] - ins["val"]
                if registers[reg] > cur_max:
                    cur_max = registers[reg]
            else:
                print("This operation isn't valid")
                continue
    return cur_max


@register
@click.command()
@click.argument("part", type=int)
def day8(part: int) -> None:
    assert part in [
        1,
        2,
    ], f"The part number {part} is not implemented, please enter either 1 or 2."
    raw_inp = parse_input(dpath)
    proc = [proc_line(line) for line in raw_inp]
    if part == 1:
        result = part_one(proc)
        print(f"The solution for part 1 is {result}")
    elif part == 2:
        result = part_two(proc)
        print(f"The solution for part 2 is {result}")
