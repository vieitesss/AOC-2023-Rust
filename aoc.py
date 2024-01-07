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
        self.builder = Builder()

    def build(self):
        try:
            self.__log_step("Setting up structure")
            self.builder.create_structure(self.__get_data_path())
        except FileExistsError as e:
            log.warning(e)
            self.update = True

        session = self.__get_session()
        self.__log_step("Downloading problem")
        self.builder.download_problem(
            self.__get_url(), session, self.__get_problem_path()
        )
        self.__log_step("Downloading examples")
        self.builder.download_examples(
            self.__get_url(), session, self.__get_data_path()
        )

        if not self.update:
            self.__log_step("Downloading input")
            self.builder.download_input(
                self.__get_input_url(), session, self.__get_input_path()
            )
            self.__log_step("Setting up program")
            self.builder.setup_program(self.__get_src_path(), self.__get_template())

        self.__log_step("Building successful")

    def __log_step(self, step: str):
        log.info(f"{step} for {self.year} day {self.day}...")

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

    def __get_url(self) -> str:
        return f"{self.URL}/{self.year}/day/{self.day}"

    def __get_input_url(self) -> str:
        return f"{self.__get_url()}/input"

    def __get_data_path(self) -> str:
        return f"{self.DATA}/aoc{self.year}/day{self.day}"


class Builder:
    def create_structure(self, path: str):
        try:
            os.makedirs(path)
        except FileExistsError:
            raise FileExistsError(f"Path {path} already exists.")

    def download_problem(self, url: str, session: Session, path: str):
        articles = self.get_articles_from_url(url, session)

        self.write_problem(articles, path)

    def download_examples(self, url: str, session: Session, path: str):
        articles = self.get_articles_from_url(url, session)

        self.write_examples(articles, path)

    def download_input(self, url: str, session: Session, input_path: str):
        text = self.get_text_from_url(url, session)

        self.write_text_in_file(text, input_path)

    def setup_program(self, path: str, template: str):
        if os.path.exists(path):
            log.warning(f"File {path} already exists.")
            return

        self.write_text_in_file(template, path)

    def write_text_in_file(self, text: str, path: str):
        with open(path, "w") as f:
            f.write(text)

    def write_problem(self, articles: list, path: str):
        for article in articles:
            self.write_text_in_file(html2text(str(article)), path)

    def write_examples(self, articles: list, path: str):
        for i, article in enumerate(articles):
            code = article.find("pre").find("code")
            self.write_text_in_file(
                self.delete_tags(str(code)), f"{path}/example{i + 1}.txt"
            )

    def get_text_from_url(self, url: str, session: Session) -> str:
        r = session.get(url)
        r.raise_for_status() # exception if status code is not 200

        return r.text

    def get_articles_from_url(self, url: str, session: Session) -> list:
        html = self.get_text_from_url(url, session)
        soup = BeautifulSoup(html, "html.parser")

        articles = soup.find_all("article")

        if articles == None:
            log.warning(f"There are no articles in the html.")
            return []

        return articles

    def delete_tags(self, text: str) -> str:
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
