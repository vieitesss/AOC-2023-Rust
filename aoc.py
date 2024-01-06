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

URL = f"https://adventofcode.com"


class AOC:
    URL = f"https://adventofcode.com"
    DATA = f"data"

    def __init__(self, year: int, day: int):
        self.year = year
        self.day = day
        self.update = False
        self.session = self.__get_session()

    def __log_step(self, step: str):
        log.info(f"{step} for {self.year} day {self.day}...")

    def build(self):
        try:
            self.__log_step("Setting up structure")
            self.__create_structure()
        except FileExistsError as e:
            log.warning(e)
            self.__set_update(True)

        self.__log_step("Downloading problem")
        self.__download_problem()

        if not self.__updating():
            self.__log_step("Downloading input")
            self.__download_input()
            self.__log_step("Setting up program")
            self.__setup_program()

        self.__log_step("Building successful")

    def __create_structure(self):
        path = self.__get_data_path()
        try:
            os.makedirs(path)
        except FileExistsError:
            raise FileExistsError(f"Path {path} already exists.")

    def __get_problem_path(self) -> str:
        return f"{self.__get_data_path()}/problem.md"

    def __get_input_path(self) -> str:
        return f"{self.__get_data_path()}/input.txt"

    def __get_session(self) -> Session:
        s = Session()
        try:
            s.cookies.set("session", os.environ["AOC_SESSION"])
        except KeyError:
            log.critical("Please set the AOC_SESSION environment variable")
            sys.exit(1)

        return s

    def __download_problem(self):
        text = self.__get_text_from_url(self.__get_url())

        articles = self.__get_articles_from_html(text)

        self.__write_problem(articles)
        self.__write_examples(articles)

    def __download_input(self) -> None:
        text = self.__get_text_from_url(f"{self.__get_url()}/input")

        self.__write_text_in_file(text, self.__get_input_path())

    def __setup_program(self):
        path = self.__get_src_path()

        if os.path.exists(path):
            log.warning(f"File {path} already exists.")
            return

        self.__write_text_in_file(self.__get_template(), path)

    def __get_template(self) -> str:
        return f"""use crate::Solution;
pub struct Day{self.day};

impl Solution for Day{self.day} {{
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

    def __get_src_path(self) -> str:
        return f"src/aoc{self.year}/day{self.day}.rs"

    def __write_text_in_file(self, text: str, path: str):
        with open(path, "w") as f:
            f.write(text)

    def __write_problem(self, articles: list):
        with open(self.__get_problem_path(), "w") as f:
            for article in articles:
                f.write(html2text(str(article)))

    def __write_examples(self, articles: list):
        for index, article in enumerate(articles):
            try:
                code = article.find("pre").find("code")
                with open(f"{self.__get_data_path()}/example{index + 1}.txt", "w") as f:
                    f.write(delete_tags(str(code)))
            except AttributeError:
                raise AttributeError(f"There is no atributte 'pre' in the article.")

    def __get_text_from_url(self, url: str) -> str:
        r = self.session.get(url)

        if r.status_code != 200:
            raise Exception(f"Could not download {url}.")

        return r.text

    def __get_articles_from_html(self, html: str) -> list:
        soup = BeautifulSoup(html, "html.parser")

        articles = soup.find_all("article")

        if articles == None:
            raise Exception(f"There are no articles in the html.")

        return articles

    def __set_update(self, update: bool):
        self.update = update

    def __updating(self) -> bool:
        return self.update

    def __get_url(self) -> str:
        return f"{self.URL}/{self.year}/day/{self.day}"

    def __get_data_path(self) -> str:
        return f"{self.DATA}/aoc{self.year}/day{self.day}"


def delete_tags(text: str) -> str:
    clean = re.compile("<.*?>")
    cleantext = re.sub(clean, "", text)

    return cleantext


@app.command()
def main(
    year: Annotated[
        int,
        typer.Argument(
            min=2019, max=2023, help="Year number 2019-2023", show_default=False
        ),
    ],
    day: Annotated[
        int,
        typer.Argument(min=1, max=25, help="Day number 1-25", show_default=False),
    ],
):
    aoc = AOC(year, day)
    aoc.build()


if __name__ == "__main__":
    app()
