import argparse
import datetime
import pathlib
import subprocess
import webbrowser

import browser_cookie3 as bc
import requests
import toml

MAIN = """\
fn main() {{
    let (part1, part2) = {crate}::solve();
    println!("{{}}", part1);
    println!("{{}}", part2);
}}\
"""

LIB = """\
use std::fmt::Display;

#[inline]
pub fn solve() -> (impl Display, impl Display) {{
    ("TODO", "TODO")
}}\
"""


def main() -> None:
    now = datetime.datetime.now()
    default_day = now.day
    default_year = now.year

    argp = argparse.ArgumentParser(description="Start solving an Advent of Code day")
    argp.add_argument(
        "-d",
        "--day",
        type=int,
        choices=range(1, 25 + 1),
        default=default_day,
        required=False,
    )
    argp.add_argument(
        "-y",
        "--year",
        type=int,
        choices=range(2015, default_year + 1),
        default=default_year,
        required=False,
    )
    argv = argp.parse_args()
    day: int = argv.day
    year: int = argv.year

    cookies = bc.load(domain_name="adventofcode.com")

    crate = f"day{day:02}"
    crate_path = pathlib.Path(crate)

    if crate_path.exists():
        print(f"{crate} already exists.")
        return

    resp = requests.get(
        f"https://adventofcode.com/{year}/day/{day}/input",
        cookies=cookies,
        headers={"User-Agent": "PurpleMyst/aoc-template getting the input! <3"},
    )
    resp.raise_for_status()
    puzzle_input = resp.text

    with open("Cargo.toml") as manifest_f:
        manifest = toml.load(manifest_f)

    manifest["workspace"]["members"].append(crate)

    with open("Cargo.toml", "w") as manifest_f:
        toml.dump(manifest, manifest_f)

    subprocess.run(["cargo", "new", "--bin", crate], check=True)
    subprocess.run(
        [
            "cargo",
            "add",
            "--manifest-path",
            "benchmark/Cargo.toml",
            "--path",
            crate,
            crate,
        ],
        check=True,
    )

    src = crate_path / "src"
    (src / "main.rs").write_text(MAIN.format(crate=crate), newline="\n")
    (src / "lib.rs").write_text(LIB.format(crate=crate), newline="\n")
    (src / "input.txt").write_text(puzzle_input, newline="\n")

    subprocess.run(["git", "add", crate], check=True)
    webbrowser.open_new(f"https://adventofcode.com/{year}/day/{day}")


if __name__ == "__main__":
    main()
