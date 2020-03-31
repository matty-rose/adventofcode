import argparse
import numpy
from typing import List
from pathlib import Path
from aoc.helpers import parse_input

input_dir = Path('/home/matt/Documents/AoC/python_2017/data')

def get_args():
    parser = argparse.ArgumentParser()
    parser.add_argument('-p', '--part', type=int, default=1, required=True, help='Enter 1 or 2 based on which part of the problem to run')
    args = parser.parse_args()
    return args

def get_digit_list() -> List:
    seq = parse_input(str(input_dir / 'day1.txt'))
    number_str = seq[0].strip()
    num_list = list(map(int, number_str))
    return num_list

def part_one(num_list: List) -> int:
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

def part_two(num_list: List) -> int:
    total_length = len(num_list)
    half = total_length // 2
    ret = 0
    for i, _ in enumerate(num_list):
        new_pos = (i + half) % total_length
        if num_list[i] == num_list[new_pos]:
            ret += num_list[i]
    return ret

if __name__ == '__main__':
    args = get_args()
    assert args.part in [1, 2], f"The part number {args.part} is not implemented, please enter either 1 or 2."
    digit_list = get_digit_list()
    if args.part == 1:
        final_sum = part_one(digit_list)
    else:
        final_sum = part_two(digit_list)
    print(f"The total sum for part {args.part} is {final_sum}.")
