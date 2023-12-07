#!/usr/bin/env python

import sys
import os
import re
import logging
from requests import Session
from bs4 import BeautifulSoup
from html2text import html2text


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
    logging.info(f"Downloading problem for day {number}...")

    # get html
    url = f"https://adventofcode.com/2023/day/{number}"
    r = s.get(url)
    soup = BeautifulSoup(r.text, "html.parser")

    articles = soup.find_all("article")

    if articles == None:
        logging.error(f"Could not parse https://adventofcode.com/2023/day/{number}.")
        return


    for index, article in enumerate(articles):
        with open(f"data/day{number}/example{index + 1}.txt", "w") as f:
            code = article.find("pre").find("code")
            f.write(delete_tags(str(code)))

    with open(f"data/day{number}/problem.txt", "w") as f:
        for article in articles:
            f.write(html2text(str(article)))

    logging.info(f"Problem for day {number} downloaded.")

def delete_tags(text: str) -> str:
    clean = re.compile('<.*?>')
    cleantext = re.sub(clean, '', text)

    return cleantext


def download_input(number: int):
    logging.info(f"Downloading input for day {number}...")
    # get html
    url = f"https://adventofcode.com/2023/day/{number}/input"
    r = s.get(url)

    # write text to file /data/day{number}/input.txt
    with open(f"data/day{number}/input.txt", "w") as f:
        f.write(r.text)

    logging.info(f"Input for day {number} downloaded.")


def setup_structure(number: int):
    logging.info(f"Setting up structure for day {number}...")
    # the number must be between 1 and 25
    if number < 1 or number > 25:
        print("The argument must be between 1 and 25")
        sys.exit(1)

    # create the folder structure
    data_path = f"data/day{number}"
    try:
        os.makedirs(data_path)
        logging.info(f"Structure for day {number} set up.")
    except FileExistsError:
        logging.info(f"Data path for day {number} already exists.")


def setup_data(number: int):
    download_problem(number)
    download_input(number)


def setup_program(number: int):
    logging.debug(f"Setting up program for day {number}")

    template = f"""use std::fs::read_to_string;

const INPUT_PATH: &str = "data/day{number}/input.txt";
const EXAMPLE1_PATH: &str = "data/day{number}/example1.txt";
const EXAMPLE2_PATH: &str = "data/day{number}/example2.txt";

pub fn resolve() {{
    println!("Hello day {number}");

    println!("Part 1: {{}}", part1());
    println!("Part 2: {{}}", part2());
}}

fn part_1() -> u32 {{
    todo!();
}}

fn part_2() -> u32 {{
    todo!();
}}"""

    # write template to file /days/day{number}/solution.rs
    with open(f"src/day{number}.rs", "w") as f:
        f.write(template)

    logging.info(f"Program for day {number} set up.")


if __name__ == "__main__":
    # the user has to provide a number as an argument
    if len(sys.argv) != 2:
        print("Usage: %s number" % sys.argv[0])
        sys.exit(1)

    # try to convert the argument to an integer
    try:
        number = int(sys.argv[1])
    except ValueError:
        logging.critical("The argument must be an integer")
        sys.exit(1)

    setup_structure(number)
    setup_data(number)
    setup_program(number)
