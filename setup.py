#!/usr/bin/env python

import sys
import os
import re
import logging
from rich.logging import RichHandler
from requests import Session
from bs4 import BeautifulSoup
from html2text import html2text
import typer
from typing_extensions import Annotated


app = typer.Typer()

logging.basicConfig(
    level=logging.INFO,
    format=f"%(message)s",
    handlers=[
        RichHandler(
            show_path=False,
            rich_tracebacks=True,
            markup=True,
            show_time=False,
        )
    ],
)

log = logging.getLogger("rich")

s = Session()
try:
    s.cookies.set("session", os.environ["AOC_SESSION"])
except KeyError:
    log.critical("Please set the AOC_SESSION environment variable")
    sys.exit(1)


def download_problem(number: int) -> None:
    """Downloads the problem for the given day and writes it to /data/day{number}/problem.md
    and each example to /data/day{number}/example{index}.txt"""
    url = f"https://adventofcode.com/2023/day/{number}"
    r = s.get(url)

    if r.status_code != 200:
        raise Exception(
            f"Could not download https://adventofcode.com/2023/day/{number}."
        )

    soup = BeautifulSoup(r.text, "html.parser")

    articles = soup.find_all("article")

    if articles == None:
        raise Exception(f"Could not parse https://adventofcode.com/2023/day/{number}.")

    with open(f"data/day{number}/problem.md", "w") as f:
        for article in articles:
            f.write(html2text(str(article)))

    for index, article in enumerate(articles):
        try:
            code = article.find("pre").find("code")
            with open(f"data/day{number}/example{index + 1}.txt", "w") as f:
                f.write(delete_tags(str(code)))
        except AttributeError:
            raise AttributeError(f"There is no atributte 'pre' in the article.")


def delete_tags(text: str) -> str:
    """Deletes all html tags from the given text."""
    clean = re.compile("<.*?>")
    cleantext = re.sub(clean, "", text)

    return cleantext


def download_input(number: int) -> None:
    """Downloads the input for the given day and writes it to /data/day{number}/input.txt"""
    # get html
    url = f"https://adventofcode.com/2023/day/{number}/input"
    r = s.get(url)

    if r.status_code != 200:
        raise Exception(
            f"Could not download https://adventofcode.com/2023/day/{number}."
        )

    # write text to file /data/day{number}/input.txt
    with open(f"data/day{number}/input.txt", "w") as f:
        f.write(r.text)


def setup_structure(number: int) -> None:
    """Creates the following structure:
    ./data/day{number}/
    """
    data_path = f"data/day{number}"
    try:
        os.makedirs(data_path)
    except FileExistsError:
        raise FileExistsError(f"Data path for day {number} already exists.")


def setup_program(number: int) -> None:
    """Creates the following structure:
    ./src/day{number}.rs
    """
    if os.path.exists(f"src/day{number}.rs"):
        raise FileExistsError(f"Program for day {number} already exists.")

    template = f"""use crate::Solution;

pub struct Day{number};

impl Solution for Day{number} {{
    type ParsedInput = String;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {{
        todo!()
    }}

    fn part_1(parsed_input: &Self::ParsedInput) -> String {{
        todo!()
    }}

    fn part_2(parsed_input: Self::ParsedInput) -> String {{
        todo!()
    }}
}}"""

    # write template to file /days/day{number}/solution.rs
    with open(f"src/day{number}.rs", "w") as f:
        f.write(template)


@app.command()
def main(
    number: Annotated[
        int,
        typer.Argument(min=1, max=25, help="Day number 1-25", show_default=False),
    ]
):
    """Sets up the structure for the given day and downloads the problem and input."""
    update = False
    try:
        log.info(f"Setting up structure for day {number}...")
        setup_structure(number)
    except FileExistsError as e:
        log.warning(e)
        update = True

    try:
        log.info(f"Downloading problem for day {number}...")
        download_problem(number)
    except AttributeError as e:
        log.warning(e)
    except Exception as e:
        log.error(e, exc_info=True)
        exit(1)

    if not update:
        try:
            log.info(f"Downloading input for day {number}...")
            download_input(number)
            log.info(f"Setting up program for day {number}")
            setup_program(number)
        except Exception as e:
            log.error(e, exc_info=True)
            sys.exit(1)

    log.info(f"Day {number} {'updated' if update else 'set up'} successfuly.")


if __name__ == "__main__":
    app()
