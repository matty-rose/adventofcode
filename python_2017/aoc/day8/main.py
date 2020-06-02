import os
from pathlib import Path

from aoc.helpers import DATA_DIR, parse_input

dpath = DATA_DIR / "day8.txt"


def proc_line(line):
    line = line.strip()
    split = line.split(" ")
    ret = {
        "register": split[0],
        "op": split[1],
        "val": int(split[2]),
        "cond": " ".join(split[3:]),
    }
    return ret


def part_one(instructions):
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


def part_two(instructions):
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


if __name__ == "__main__":
    raw_inp = parse_input(dpath)
    proc = [proc_line(line) for line in raw_inp]
    print(part_one(proc))
    print(part_two(proc))
