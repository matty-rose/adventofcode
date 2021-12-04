from functools import reduce

import click

from aoc.helpers import DATA_DIR, parse_input
from aoc.registry import register

data = DATA_DIR / "day10.txt"


def convert_str_to_ascii(s: str, end: tuple[int, ...] = (17, 31, 73, 47, 23)) -> list[int]:
    ret_list = []
    for char in s:
        ret_list.append(ord(char))
    for val in end:
        ret_list.append(val)
    return ret_list


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


def make_dense(vals: list[int], n: int = 16) -> list[int]:
    dense = []
    num_blocks = int(256 / n)
    for b in range(num_blocks):
        block = vals[b * n : b * n + n]
        bit = reduce(lambda x, y: x ^ y, block)
        dense.append(bit)
    return dense


def convert_to_hex(dense: list[int]) -> str:
    ret = ""
    for i in dense:
        ret += hex(i)[2:].zfill(2)
    return ret


def part_one(lens: list[int]) -> int:
    max_val = 256
    vals = list(range(max_val))
    pos = 0
    skip = 0
    for length in lens:
        vals = vals[:max_val] * 2
        vals[pos : pos + length] = reversed(vals[pos : pos + length])
        remain = (pos + length) % max_val
        if pos + length >= max_val:
            vals[:remain] = vals[max_val : max_val + remain]
        pos = (pos + length + skip) % (max_val)
        skip += 1
    return vals[0] * vals[1]


def part_two(s: str) -> str:
    lens = convert_str_to_ascii(s)
    final_vals = do_len_loops(lens, num_loops=64)
    dense_hash = make_dense(final_vals)
    hex_string = convert_to_hex(dense_hash)
    return hex_string


@register
@click.command()
@click.argument("part", type=int)
def day10(part: int) -> None:
    assert part in [
        1,
        2,
    ], f"The part number {part} is not implemented, please enter either 1 or 2."
    raw = parse_input(data)
    raw_ints = [int(i) for i in raw[0].strip().split(",")]
    if part == 1:
        result = part_one(raw_ints)
        print(f"The product of the first two numbers is {result}")
    elif part == 2:
        result = part_two(raw[0].strip())  # type: ignore
        print(f"The hexadecimal representation is {result}")
