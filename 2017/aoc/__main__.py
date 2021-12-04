import click
from aoc.day1.main import day1


@click.group()
def cli():
    pass


cli.add_command(day1)

if __name__ == "__main__":
    cli()
