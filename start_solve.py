import argparse
import datetime
import subprocess
import webbrowser
from functools import partial
from pathlib import Path

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

run = partial(subprocess.run, check=True)


def add_line(p: Path, l: str) -> None:
    ls = p.read_text().splitlines()
    ls.insert(-1, l)
    p.write_text("\n".join(ls), newline="\n")


def argument_parser() -> argparse.ArgumentParser:
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
    return argp


def main() -> None:
    argv = argument_parser().parse_args()
    day: int = argv.day
    year: int = argv.year

    cookies = bc.load(domain_name="adventofcode.com")

    crate = f"day{day:02}"
    crate_path = Path(crate)

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

    run(("cargo", "new", "--bin", crate))
    run(
        (
            "cargo",
            "add",
            "--manifest-path",
            "benchmark/Cargo.toml",
            "--path",
            crate,
            crate,
        )
    )

    src = crate_path / "src"
    (src / "main.rs").write_text(MAIN.format(crate=crate), newline="\n")
    (src / "lib.rs").write_text(LIB.format(crate=crate), newline="\n")
    (src / "input.txt").write_text(puzzle_input, newline="\n")

    benches = Path("benchmark", "benches")
    add_line(benches / "criterion.rs", f"{crate},")
    add_line(benches / "iai.rs", f"{crate}: {crate}_solve,")

    run(("git", "add", crate))
    webbrowser.open_new(f"https://adventofcode.com/{year}/day/{day}")


if __name__ == "__main__":
    main()
