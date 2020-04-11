import os
from pathlib import Path
from aoc.helpers import parse_input, DATA_DIR
from typing import List

data = DATA_DIR / "day7.txt"


class Node:
    def __init__(self, name: str, data: int, parent=None, children: List = []):
        self.name = name
        self.data = data
        self.parent = parent
        self.children = children


class Tree:
    def __init__(self, root_name: str, tower_dict: dict):
        self.data = tower_dict
        self.root = Node(root_name, self.data[root_name]["weight"])
        self.root.children = self.make_children(self.root)

    def make_children(self, node: Node):
        child_names = self.data[node.name]["above"]
        child_nodes = []
        if len(child_names) != 0:
            for c in child_names:
                new_node = Node(c, self.data[c]["weight"], node)
                new_node.children = self.make_children(new_node)
                child_nodes.append(new_node)
        return child_nodes


def process_raw(raw):
    stripped = [p.strip() for p in raw]
    ret = {}
    for line in stripped:
        els = line.split(" ")
        name = els[0]
        weight = int(els[1].rstrip(")").lstrip("("))
        above = [el.rstrip(",") for el in els[3:]] if len(els) > 2 else []
        ret[name] = {"weight": weight, "above": above}
    return ret


def find_root(program_dict):
    children = [val["above"] for val in program_dict.values()]
    children_flattened = set([c for l in children for c in l])
    programs = set(program_dict.keys())
    root = programs - children_flattened
    return root.pop()


def get_sum_above(node):
    above_sum = 0
    for c in node.children:
        above_sum += c.data + get_sum_above(c)
    return above_sum


def make_tree(program_dict):
    root = find_root(program_dict)
    tree = Tree(root, program_dict)
    return tree


def part_one(proc):
    bottom_program = find_root(proc)
    return bottom_program


def part_two(proc):
    tree = make_tree(proc)
    cur_node = tree.root
    sums = [c.data + get_sum_above(c) for c in cur_node.children]
    prev_sums = sums.copy()
    while True:
        print(sums)
        if len(set(sums)) == 1:
            diff = prev_sums[bad_idx] - prev_sums[bad_idx - 1 % 3]
            return cur_node.data - diff
        else:
            prev_sums = sums.copy()
            bad_sum = filter(lambda x: sums.count(x) == 1, sums)
            bad_idx = sums.index(list(bad_sum)[0])
            cur_node = cur_node.children[bad_idx]
            sums = [c.data + get_sum_above(c) for c in cur_node.children]
    return None


if __name__ == "__main__":
    raw_inp = parse_input(data)
    proc = process_raw(raw_inp)
    print(f"The name of the bottom program is {part_one(proc)}.")
    print(part_two(proc))
