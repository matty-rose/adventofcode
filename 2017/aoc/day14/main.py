import numpy as np
from functools import reduce
from pathlib import Path
from typing import List

INP = "uugsqrei"


def convert_str_to_ascii(s: str, end: List[int] = [17, 31, 73, 47, 23]):
    ret_list = []
    for char in s:
        ret_list.append(ord(char))
    for val in end:
        ret_list.append(val)
    return ret_list


def make_dense(vals, n=16):
    dense = []
    num_blocks = int(256 / n)
    for b in range(num_blocks):
        block = vals[b * n : b * n + n]
        bit = reduce(lambda x, y: x ^ y, block)
        dense.append(bit)
    return dense


def do_len_loops(lens, num_loops):
    max_val = 256
    vals = list(range(max_val))
    pos, skip = 0, 0
    for _ in range(num_loops):
        for l in lens:
            vals = vals[:max_val] * 2
            vals[pos : pos + l] = reversed(vals[pos : pos + l])
            remain = (pos + l) % max_val
            if pos + l >= max_val:
                vals[:remain] = vals[max_val : max_val + remain]
            pos = (pos + l + skip) % max_val
            skip += 1
    return vals[:max_val]


def convert_to_hex(dense):
    ret = ""
    for i in dense:
        ret += hex(i)[2:].zfill(2)
    return ret


def calc_knot_hash(s: str) -> List[str]:
    ascii_list = convert_str_to_ascii(s)
    looped = do_len_loops(ascii_list, 64)
    dense_hash = make_dense(looped)
    knot_hash = convert_to_hex(dense_hash)
    return knot_hash


def knot_hash_to_bits(kh: str) -> np.array:
    ret = np.array(
        [list(map(int, bin(int(ch, 16))[3:].zfill(4))) for ch in kh]
    ).flatten()
    return ret


def part_one(inp: str):
    kh = calc_knot_hash(inp)
    bits = np.array(
        [knot_hash_to_bits(calc_knot_hash(f"{inp}-{i}")) for i in range(128)]
    )
    return bits.sum()


def part_two(inp: str):
    pass


if __name__ == "__main__":
    print(part_one(INP))
    print(part_two(INP))
