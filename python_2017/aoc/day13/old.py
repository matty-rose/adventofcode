import os
from copy import deepcopy
from pathlib import Path

from aoc.helpers import DATA_DIR, parse_input

data = DATA_DIR / "day13.txt"
# data = DATA_DIR / "day13test.txt"


def proc_raw(raw):
    ret = {}
    for line in raw:
        line = line.strip()
        d, r = line.split(": ")
        d, r = int(d), [[1] + [0 for _ in range(int(r) - 1)], True]
        ret[d] = r
    return ret


def update_states(s):
    for i in s:
        cur_idx = s[i][0].index(1)
        new_idx = (cur_idx + 1) if s[i][1] else (cur_idx - 1)
        if new_idx == 0 or new_idx == len(s[i][0]) - 1:
            s[i][1] = not s[i][1]
        s[i][0][cur_idx] = 0
        s[i][0][new_idx] = 1
    return s


def part_one(proc):
    states = proc.copy()
    num_layers = max(states.keys())
    severity = 0
    for ps in range(num_layers + 1):
        if ps in states:
            if states[ps][0][0] == 1:
                severity += ps * len(states[ps][0])
        states = update_states(states)
    return severity


""" This solution is way too slow - too inefficient storing and calculating the places of each scanner at every time step"""
# def part_two(proc):
#     delay_starts = {}
#     states = deepcopy(proc)
#     for i in range(100000):
#         delay_starts[i] = states
#         states = update_states(states)
#     delay = 0
#     while True:
#         start = delay_starts[delay]
#         caught = False
#         for ps in range(max(start.keys()) + 1):
#             if ps in start:
#                 if start[ps][0][0] == 1:
#                     caught = True
#             start = update_states(start)
#         if caught == False:
#             break
#         else:
#             delay += 1
#     return delay

if __name__ == "__main__":
    raw = parse_input(data)
    proc = proc_raw(raw)
    print(part_one(deepcopy(proc)))
    # print(part_two(deepcopy(proc)))
