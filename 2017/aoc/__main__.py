import click
from aoc.days import *
from aoc.registry import COMMAND_REGISTRY


@click.group()
def cli() -> None:
    pass


for _name, command in COMMAND_REGISTRY.items():
    cli.add_command(command)


if __name__ == "__main__":
    cli()
