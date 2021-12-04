from click import Command

COMMAND_REGISTRY: dict[str, Command] = {}


def register(func: Command) -> Command:
    COMMAND_REGISTRY[func.name] = func
    return func
