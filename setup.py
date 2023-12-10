#!/usr/bin/env python

from shutil import Error
import sys
import os
import re
import logging
from requests import Session
from bs4 import BeautifulSoup
from html2text import html2text
import typer
from typing_extensions import Annotated


app = typer.Typer()

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s [%(levelname)s] %(message)s",
    handlers=[logging.StreamHandler(sys.stdout)],
)

s = Session()
try:
    s.cookies.set("session", os.environ["AOC_SESSION"])
except KeyError:
    logging.critical("Please set the AOC_SESSION environment variable")
    sys.exit(1)


def download_problem(number: int):
    """Downloads the problem for the given day and writes it to /data/day{number}/problem.md
    and each example to /data/day{number}/example{index}.txt"""
    url = f"https://adventofcode.com/2023/day/{number}"
    r = s.get(url)

    if r.status_code != 200:
        raise Error(f"Could not download https://adventofcode.com/2023/day/{number}.")

    soup = BeautifulSoup(r.text, "html.parser")

    articles = soup.find_all("article")

    if articles == None:
        raise Error(f"Could not parse https://adventofcode.com/2023/day/{number}.")

    for index, article in enumerate(articles):
        with open(f"data/day{number}/example{index + 1}.txt", "w") as f:
            code = article.find("pre").find("code")
            f.write(delete_tags(str(code)))

    with open(f"data/day{number}/problem.md", "w") as f:
        for article in articles:
            f.write(html2text(str(article)))


def delete_tags(text: str) -> str:
    """Deletes all html tags from the given text."""
    clean = re.compile("<.*?>")
    cleantext = re.sub(clean, "", text)

    return cleantext


def download_input(number: int):
    """Downloads the input for the given day and writes it to /data/day{number}/input.txt"""
    # get html
    url = f"https://adventofcode.com/2023/day/{number}/input"
    r = s.get(url)

    if r.status_code != 200:
        raise Error(f"Could not download https://adventofcode.com/2023/day/{number}.")

    # write text to file /data/day{number}/input.txt
    with open(f"data/day{number}/input.txt", "w") as f:
        f.write(r.text)


def setup_structure(number: int):
    """Creates the following structure:
    ./data/day{number}/
    """
    data_path = f"data/day{number}"
    try:
        os.makedirs(data_path)
    except FileExistsError:
        raise FileExistsError(f"Data path for day {number} already exists.")


def setup_program(number: int):
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

    fn part_1(parsed_input: &mut Self::ParsedInput) -> String {{
        todo!()
    }}

    fn part_2(parsed_input: &mut Self::ParsedInput) -> String {{
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
        logging.info(f"Setting up structure for day {number}...")
        setup_structure(number)
        logging.info(f"Structure for day {number} set up.")
    except FileExistsError as e:
        logging.error(e)
        update = True

    try:
        logging.info(f"Downloading problem for day {number}...")
        download_problem(number)
        logging.info(f"Problem for day {number} downloaded.")
    except Error as e:
        logging.error(e)
        sys.exit(1)

    if not update:
        try:
            logging.info(f"Downloading problem for day {number}...")
            download_problem(number)
            logging.info(f"Problem for day {number} downloaded.")
            logging.info(f"Downloading input for day {number}...")
            download_input(number)
            logging.info(f"Input for day {number} downloaded.")
            logging.debug(f"Setting up program for day {number}")
            setup_program(number)
            logging.info(f"Program for day {number} set up.")
            logging.info(f"Downloading problem for day {number}...")
            download_problem(number)
            logging.info(f"Problem for day {number} downloaded.")
        except Error as e:
            logging.error(e)
            sys.exit(1)


if __name__ == "__main__":
    app()
