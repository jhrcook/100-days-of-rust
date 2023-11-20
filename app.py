#!/usr/bin/env python

# Command line tool for logging a day's work.
#
# Currently, this tool is written in Python, but I should be able to switch to Rust
# before the end of the 100 days.

from pathlib import Path
from typing import Annotated

import sh
import typer
from loguru import logger

app = typer.Typer(add_completion=False, rich_markup_mode="rich")


def _get_next_line_insertion_index(lines: list[str]) -> int:
    for i, line in enumerate(lines):
        if "[[NEXT DAY]]" in line:
            return i
    raise ValueError("Could not locate line indicating the next day's entry location.")


def _extract_last_day_num(text: str) -> int:
    return int(text.split("|")[1].strip())


def update_readme(msg: str, skip_confirm: bool = False) -> int:
    """Update the README with today's activity.

    Args:
        msg (str): Activity message describing what was learned.

    Returns:
        int: Current day of the 100 Days.
    """
    file = Path("./README.md")
    with open(file, "r") as f:
        lines = f.readlines()
    i = _get_next_line_insertion_index(lines)
    day_num = _extract_last_day_num(lines[i - 1]) + 1
    new_line = f"| {day_num: 3d} | {msg.capitalize()} |\n"
    lines.insert(i, new_line)
    write_lines = skip_confirm or typer.confirm(
        "Write new line to README?", default=True
    )
    if write_lines:
        with open("README.md", "w") as f:
            f.writelines(lines)
    else:
        logger.info("No changes to README made.")
    return day_num


def commit_to_git(
    msg: str, day_num: int, git_add_all: bool = True, skip_confirm: bool = False
) -> None:
    """Make a git commit.

    Args:
        msg (str): Commit message body.
        day_num (int): Current day of the 100 Days.
    """
    if git_add_all:
        sh.git.add("--all")
    print(sh.git.status())
    commit_msg = f"day {day_num}: {msg.strip()}"
    make_commit = skip_confirm or typer.confirm(
        f"Make git commit? '{commit_msg}'", default=True
    )
    if make_commit:
        sh.git.commit("-m", msg.strip())
    else:
        logger.info("No git commit made.")

    if make_commit and (skip_confirm or typer.confirm("Push to origin?", default=True)):
        sh.git.push()


@app.command()
def main(
    msg: Annotated[str, typer.Argument(help="Text describing what was learned today.")],
    no_git_add: Annotated[
        bool,
        typer.Option(
            "--no-git-add",
            help="No additions to git staging (defaults to adding all changes.)",
        ),
    ] = False,
    yes: Annotated[
        bool, typer.Option("--yes", "-y", is_flag=True, flag_value=True)
    ] = False,
) -> None:
    """[b]Log a day of 100 Days of Rust.[b]"""
    logger.info("Updating README.")
    day_num = update_readme(msg, skip_confirm=yes)
    logger.info(f"Making git commit for day {day_num}.")
    commit_to_git(msg, day_num=day_num, git_add_all=not no_git_add, skip_confirm=yes)


if __name__ == "__main__":
    app()
