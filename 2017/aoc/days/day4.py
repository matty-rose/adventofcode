import click

from aoc.helpers import DATA_DIR, parse_input
from aoc.registry import register

INP = DATA_DIR / "day4.txt"


def validate_passphrase(pp: list[str]) -> bool:
    phrase_len = len(pp)
    for i in range(phrase_len - 1):
        for j in range(i + 1, phrase_len):
            if pp[i] == pp[j]:
                return False
    return True


def check_anagram(w1: str, w2: str) -> bool:
    chars_1 = [c for c in w1]
    chars_2 = [c for c in w2]
    if sorted(chars_1) == sorted(chars_2):
        return True
    return False


def validate_p2(pp: list[str]) -> bool:
    phrase_len = len(pp)
    for i in range(phrase_len - 1):
        for j in range(i + 1, phrase_len):
            if check_anagram(pp[i], pp[j]):
                return False
    return True


def part_one() -> int:
    raw_inp = parse_input(INP)
    stripped_and_split = [p.strip().split(" ") for p in raw_inp]
    valid_count = 0
    for passphrase in stripped_and_split:
        if validate_passphrase(passphrase):
            valid_count += 1
    return valid_count


def part_two() -> int:
    raw_inp = parse_input(INP)
    stripped_and_split = [p.strip().split(" ") for p in raw_inp]
    valid_count = 0
    for pp in stripped_and_split:
        if validate_p2(pp):
            valid_count += 1
    return valid_count


@register
@click.command()
@click.argument("part", type=int)
def day4(part: int) -> None:
    assert part in [
        1,
        2,
    ], f"The part number {part} is not implemented, please enter either 1 or 2."
    if part == 1:
        result = part_one()
        print(f"There are {result} valid passphrases in the input")
    elif part == 2:
        result = part_two()
        print(f"There are {result} valid passphrases for part 2 in the input")
    else:
        raise NotImplementedError()
