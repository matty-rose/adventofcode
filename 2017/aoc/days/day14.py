from functools import reduce
from typing import Any

import click
import numpy as np

from aoc.registry import register

INP = "uugsqrei"


def convert_str_to_ascii(s: str, end: tuple[int, ...] = (17, 31, 73, 47, 23)) -> list[int]:
    ret_list = []
    for char in s:
        ret_list.append(ord(char))
    for val in end:
        ret_list.append(val)
    return ret_list


def make_dense(vals: list[int], n: int = 16) -> list[int]:
    dense = []
    num_blocks = int(256 / n)
    for b in range(num_blocks):
        block = vals[b * n : b * n + n]
        bit = reduce(lambda x, y: x ^ y, block)
        dense.append(bit)
    return dense


def do_len_loops(lens: list[int], num_loops: int) -> list[int]:
    max_val = 256
    vals = list(range(max_val))
    pos, skip = 0, 0
    for _ in range(num_loops):
        for length in lens:
            vals = vals[:max_val] * 2
            vals[pos : pos + length] = reversed(vals[pos : pos + length])
            remain = (pos + length) % max_val
            if pos + length >= max_val:
                vals[:remain] = vals[max_val : max_val + remain]
            pos = (pos + length + skip) % max_val
            skip += 1
    return vals[:max_val]


def convert_to_hex(dense: list[int]) -> str:
    ret = ""
    for i in dense:
        ret += hex(i)[2:].zfill(2)
    return ret


def calc_knot_hash(s: str) -> str:
    ascii_list = convert_str_to_ascii(s)
    looped = do_len_loops(ascii_list, 64)
    dense_hash = make_dense(looped)
    knot_hash = convert_to_hex(dense_hash)
    return knot_hash


def knot_hash_to_bits(kh: str) -> np.ndarray:  # type: ignore[type-arg]
    ret = np.array([list(map(int, bin(int(ch, 16))[3:].zfill(4))) for ch in kh]).flatten()
    return ret


def part_one(inp: str) -> Any:
    kh = calc_knot_hash(inp)
    bits = np.array([knot_hash_to_bits(calc_knot_hash(f"{inp}-{i}")) for i in range(128)])
    return bits.sum()


def part_two(inp: str):
    pass


@register
@click.command()
@click.argument("part", type=int)
def day14(part: int) -> None:
    assert part in [
        1,
    ], f"The part number {part} is not implemented, please enter either 1 or 2."
    if part == 1:
        result = part_one(INP)
        print(f"The solution to part 1 is {result}")
    elif part == 2:
        result = part_two(INP)
        print(f"The solution to part 2 is {result}")
