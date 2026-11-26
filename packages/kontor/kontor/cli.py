import logging
from pathlib import Path
from typing import Any

import click

from . import Kontor


class ClickLoggingHandler(logging.Handler):
    def __init__(self, level: int | str = 0) -> None:
        super().__init__(level)
        self._prefix = click.style("(kontor)", fg="white")
        self._levels = {
            logging.DEBUG: click.style("debug:", fg="blue"),
            logging.INFO: click.style("info:", fg="green"),
            logging.WARNING: click.style("info:", fg="yellow"),
            logging.ERROR: click.style("info:", fg="red"),
        }

    def emit(self, record: logging.LogRecord) -> None:
        try:
            level = self._levels.get(record.levelno, record.levelname.lower() + ":")
            message = record.getMessage()
            click.echo(f"{self._prefix} {level} {message}")
        except Exception:  # noqa: BLE001
            self.handleError(record)


def debug_option():
    def enable_debug_logging(ctx: Any, param: Any, value: bool):
        if value and logging.root.level > logging.DEBUG:
            logging.root.setLevel(logging.DEBUG)

    return click.option(
        "--debug",
        is_flag=True,
        expose_value=False,
        show_envvar=True,
        help="Enable debug logging.",
        callback=enable_debug_logging,
    )


@click.group(
    context_settings={
        "auto_envvar_prefix": "kontor",
        "help_option_names": ["-h", "--help"],
    }
)
@debug_option()
def cli() -> None:
    """Declarative Nix user environment."""
    logging.root.addHandler(ClickLoggingHandler())
    if logging.root.level > logging.INFO:
        logging.root.setLevel(logging.INFO)


@cli.command
@debug_option()
@click.argument("path", type=click.Path(path_type=Path))
def link(path: Path) -> None:
    """Links a file into the Kontor."""
    Kontor().link(path)


@cli.command
@debug_option()
def switch() -> None:
    """Build the user environment and switch to it."""
    Kontor().switch()


if __name__ == "__main__":
    cli()
