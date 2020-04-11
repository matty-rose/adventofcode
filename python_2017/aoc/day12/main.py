import os
from pathlib import Path
from aoc.helpers import parse_input, DATA_DIR
import random

data = DATA_DIR / "day12.txt"

def proc_raw(raw):
    ret = {}
    for l in raw:
        l = l.strip().split(" <-> ")
        prog = int(l[0])
        comms = [int(i) for i in l[1].split(", ")]
        ret[prog] = comms
    return ret

def get_connected(program, data, current_list):
    children = data[program]
    for c in children:
        if c in current_list:
            continue
        else:
            current_list.append(c)
            current_list = get_connected(c, data, current_list)
    return current_list

def part_one(proc, p=0):
    all_progs = [p]
    all_connected = set(get_connected(p, proc, all_progs))
    return len(all_connected)

def part_two(proc):
    num_groups = 0
    remaining_progs = set(proc.keys())
    while len(remaining_progs) != 0:
        el = random.choice(list(remaining_progs))
        el_group = set(get_connected(el, proc, [el]))
        remaining_progs = remaining_progs - el_group
        num_groups += 1
    return num_groups

if __name__ == "__main__":
    raw = parse_input(data)
    proc = proc_raw(raw)
    print(part_one(proc))
    print(part_two(proc))
