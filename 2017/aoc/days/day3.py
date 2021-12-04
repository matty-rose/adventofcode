from typing import Any

import click
import numpy as np

from aoc.registry import register

INP = 289326

"""
1: (0,0)
9: (1,-1)
25: (2,-2)
49: (3,-3)
"""

Point = tuple[float, float]


def round_up_to_odd(num: float) -> Any:
    return np.ceil(num) // 2 * 2 + 1


def manhattan_dist(pt1: Point, pt2: Point) -> Any:
    dist = np.abs(pt1[0] - pt2[0]) + np.abs(pt1[1] - pt2[1])
    return dist


def calc_square(br: int) -> dict[int, Point]:
    coord = (br - 1) / 2
    outer_square_dict = {br ** 2: (coord, -coord)}
    cur_y = -coord
    cur_x = coord
    pt = br ** 2

    while cur_x > -coord:
        pt -= 1
        cur_x -= 1
        outer_square_dict[pt] = (cur_x, cur_y)

    while cur_y < coord:
        pt -= 1
        cur_y += 1
        outer_square_dict[pt] = (cur_x, cur_y)

    while cur_x < coord:
        pt -= 1
        cur_x += 1
        outer_square_dict[pt] = (cur_x, cur_y)

    while cur_y > (-coord + 1):
        pt -= 1
        cur_y -= 1
        outer_square_dict[pt] = (cur_x, cur_y)
    return outer_square_dict


def get_neighbour_sum(pt: Point, val_dict: dict[Any, Any]) -> float:
    pt_x, pt_y = pt
    valid_xs = [pt_x - 1, pt_x, pt_x + 1]
    valid_ys = [pt_y - 1, pt_y, pt_y + 1]
    neighbours = {
        k: v for k, v in val_dict.items() if (k[0] in valid_xs) and (k[1] in valid_ys) and k != pt
    }
    val_sum = sum(neighbours.values())
    return val_sum


def part_one(sq: int) -> Any:
    sq_root = np.sqrt(sq)
    upper_closest_odd = round_up_to_odd(sq_root)
    outer_square = calc_square(upper_closest_odd)
    final_dist = manhattan_dist(outer_square[INP], (0, 0))
    return final_dist


def part_two(INP: int) -> float:
    mat: dict[Point, float] = {(0, 0): 1}
    max_depth, cur_x, cur_y = 0, 0, 0
    while True:
        while cur_x <= max_depth:
            cur_x += 1
            new_val = get_neighbour_sum((cur_x, cur_y), mat)
            if new_val > INP:
                return new_val
            mat[(cur_x, cur_y)] = new_val
        while cur_y <= max_depth:
            cur_y += 1
            new_val = get_neighbour_sum((cur_x, cur_y), mat)
            if new_val > INP:
                return new_val
            mat[(cur_x, cur_y)] = new_val
        while cur_x >= -max_depth:
            cur_x -= 1
            new_val = get_neighbour_sum((cur_x, cur_y), mat)
            if new_val > INP:
                return new_val
            mat[(cur_x, cur_y)] = new_val
        while cur_y >= -max_depth:
            cur_y -= 1
            new_val = get_neighbour_sum((cur_x, cur_y), mat)
            if new_val > INP:
                return new_val
            mat[(cur_x, cur_y)] = new_val
        max_depth += 1


@register
@click.command()
@click.argument("part", type=int)
def day3(part: int) -> None:
    assert part in [
        1,
        2,
    ], f"The part number {part} is not implemented, please enter either 1 or 2."
    if part == 1:
        result = part_one(INP)
        print(f"The manhattan distance between 1 and {INP} in the spiral matrix is {result}")
    elif part == 2:
        result = part_two(INP)
        print(f"The first value above {INP} is {result}")
    else:
        raise NotImplementedError()
