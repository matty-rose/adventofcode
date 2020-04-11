import os
from pathlib import Path

from aoc.helpers import parse_input

data = Path("data/day6.txt")


def redistribute(max_idx, blocks):
    num_banks = len(blocks)
    num_blocks = blocks[max_idx]
    blocks[max_idx] = 0
    cur_idx = max_idx + 1
    while num_blocks != 0:
        blocks[cur_idx % num_banks] = blocks[cur_idx % num_banks] + 1
        cur_idx += 1
        num_blocks -= 1
    return blocks


def part_one(inp):
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


def part_two(inp):
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
    return states


if __name__ == "__main__":
    raw_inp = parse_input(data)
    proc_inp = [int(i) for i in raw_inp[0].strip().split("\t")]
    print(part_one(proc_inp))
    print(part_two(proc_inp))
