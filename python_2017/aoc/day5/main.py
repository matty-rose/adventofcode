import os
from pathlib import Path

from aoc.helpers import parse_input

data_dir = Path('/home/matt/Documents/AoC/python_2017/data/day5.txt')

def part_one(proc_inp):
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

def part_two(proc_inp):
    pass

if __name__ == "__main__":
    raw_inp = parse_input(data_dir)
    int_inp = [int(p.strip()) for p in raw_inp]
    print(part_one(int_inp))
