import os
from pathlib import Path
from aoc.helpers import DATA_DIR, parse_input
from functools import reduce

data = DATA_DIR / "day10.txt"

def convert_str_to_ascii(s, end=[17, 31, 73, 47, 23]):
    ret_list = []
    for char in s:
        ret_list.append(ord(char))
    for val in end:
        ret_list.append(val)
    return ret_list

def do_len_loops(lens, num_loops):
    max_val = 256
    vals = list(range(max_val))
    pos, skip = 0, 0
    for _ in range(num_loops):
        for l in lens:
            vals = vals[:max_val] * 2
            vals[pos:pos + l] = reversed(vals[pos:pos + l])
            remain = (pos + l) % max_val
            if pos + l >= max_val:
                vals[:remain] = vals[max_val:max_val + remain]
            pos = (pos + l + skip) % max_val
            skip += 1
    return vals[:max_val]

def make_dense(vals, n=16):
    dense = []
    num_blocks = int(256 / n)
    for b in range(num_blocks):
        block = vals[b * n : b * n + n]
        bit = reduce(lambda x, y: x ^ y, block)
        dense.append(bit)
    return dense

def convert_to_hex(dense):
    ret = ""
    for i in dense:
        ret += hex(i)[2:].zfill(2)
    return ret

def part_one(lens):
    max_val = 256
    vals = list(range(max_val))
    pos = 0
    skip = 0
    for l in lens:
        vals = vals[:max_val] * 2
        vals[pos:pos + l] = reversed(vals[pos:pos + l])
        remain = (pos + l) % max_val
        if pos + l >= max_val:
            vals[:remain] = vals[max_val:max_val + remain]
        pos = (pos + l + skip) % (max_val)
        skip += 1
    return vals[0] * vals[1]

def part_two(s):
    lens = convert_str_to_ascii(s)
    final_vals = do_len_loops(lens, num_loops=64)
    dense_hash = make_dense(final_vals)
    hex_string = convert_to_hex(dense_hash)
    return hex_string

if __name__ == "__main__":
    raw = parse_input(data)
    raw_ints = [int(i) for i in raw[0].strip().split(",")]
    print(f"The product of the first two numbers is {part_one(raw_ints)}")
    actual_string = raw[0].strip()
    print(f"The hexadecimal representation is {part_two(actual_string)}")
