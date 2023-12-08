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

    with open(f"data/day{number}/problem.md", "w") as f:
        for article in articles:
            f.write(html2text(str(article)))

    logging.info(f"Problem for day {number} downloaded.")

def delete_tags(text: str) -> str:
    clean = re.compile('<.*?>')
    cleantext = re.sub(clean, '', text)

    return cleantext

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

    download_problem(number)
