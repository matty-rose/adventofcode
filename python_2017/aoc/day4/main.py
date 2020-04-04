import os
from pathlib import Path

from aoc.helpers import parse_input

INP = Path('/home/matt/Documents/AoC/python_2017/data/day4.txt')

def validate_passphrase(pp):
    phrase_len = len(pp)
    for i in range(phrase_len - 1):
        for j in range(i + 1, phrase_len):
            if pp[i] == pp[j]:
                return False
    return True

def check_anagram(w1, w2):
    chars_1 = [c for c in w1]
    chars_2 = [c for c in w2]
    if sorted(chars_1) == sorted(chars_2):
        return True
    return False

def validate_p2(pp):
    phrase_len = len(pp)
    for i in range(phrase_len - 1):
        for j in range(i + 1, phrase_len):
            if check_anagram(pp[i], pp[j]):
                return False
    return True

def part_one():
    raw_inp = parse_input(INP)
    stripped_and_split = [p.strip().split(" ") for p in raw_inp]
    valid_count = 0
    for passphrase in stripped_and_split:
        if validate_passphrase(passphrase):
            valid_count += 1
    return valid_count

def part_two():
    raw_inp = parse_input(INP)
    stripped_and_split = [p.strip().split(" ") for p in raw_inp]
    valid_count = 0
    for pp in stripped_and_split:
        if validate_p2(pp):
            valid_count += 1
    return valid_count

if __name__ == "__main__":
    print(f"There are {part_one()} valid passphrases in the input")
    print(f"There are {part_two()} valid passphrases for part 2 in the input")
