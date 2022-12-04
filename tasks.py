import datetime
import subprocess
import webbrowser
from functools import partial, wraps
from os import chdir, environ
from pathlib import Path

import browser_cookie3 as bc
import requests
import toml
from argh import aliases, arg, dispatch_commands, named, wrap_errors

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
pub fn solve() -> (impl Display, impl Display) {
    ("TODO", "TODO")
}\
"""

DEFAULT_BASELINE = "previous"

NOW = datetime.datetime.now()
DEFAULT_DAY = NOW.day
DEFAULT_YEAR = NOW.year

run = partial(subprocess.run, check=True)


def add_line(p: Path, l: str) -> None:
    ls = p.read_text().splitlines()
    ls.insert(-1, l)
    p.write_text("\n".join(ls), newline="\n")


def rechdir(f):
    @wraps(f)
    def inner(*args, **kwargs):
        chdir(Path(__file__).parent)
        return f(*args, **kwargs)

    return inner


@arg(
    "-d",
    "--day",
    choices=range(1, 25 + 1),
    required=False,
)
@arg(
    "-y",
    "--year",
    choices=range(2015, DEFAULT_YEAR + 1),
    required=False,
)
@aliases("ss")
@wrap_errors((requests.HTTPError,))
def start_solve(day: int = DEFAULT_DAY, year: int = DEFAULT_YEAR) -> None:
    "Start solving a day, by default today."
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
    (src / "lib.rs").write_text(LIB, newline="\n")
    (src / "input.txt").write_text(puzzle_input, newline="\n")

    benches = Path("benchmark", "benches")
    add_line(benches / "criterion.rs", f"    {crate},")
    add_line(benches / "iai.rs", f"    {crate}: {crate}_solve,")

    run(("git", "add", crate))
    webbrowser.open_new(f"https://adventofcode.com/{year}/day/{day}")


@aliases("sb")
@rechdir
def set_baseline(day: str, name: str = DEFAULT_BASELINE) -> None:
    "Run a criterion benchmark, setting its results as the new baseline."
    run(
        (
            "cargo",
            "bench",
            "--bench",
            "criterion",
            "--",
            day,
            "--save-baseline",
            name,
            "--verbose",
        )
    )


@aliases("cmp")
@rechdir
def compare(day: str, name: str = DEFAULT_BASELINE) -> None:
    "Run a criterion benchmark, comparing its results to the saved baseline."
    run(
        (
            "cargo",
            "bench",
            "--bench",
            "criterion",
            "--",
            day,
            "--baseline",
            name,
            "--verbose",
        )
    )


@rechdir
def criterion(day: str) -> None:
    "Run a criterion benchmark, without caring about baselines."
    run(("cargo", "bench", "--bench", "criterion", "--", day, "--verbose"))


@rechdir
def iai() -> None:
    "Run the iai benchmark."
    run(("cargo", "bench", "--bench", "iai"))


@aliases("wr")
def watch_run() -> None:
    "Run the solution everytime it changes."
    run(("cargo", "watch", "--clear", "--exec", "run"))


@aliases("r")
@named("run")
def do_run() -> None:
    "Run the solution."
    run(("cargo", "run"))


@aliases("rr")
def run_release() -> None:
    "Run the solution, in release mode."
    run(("cargo", "run", "--release"))


@aliases("rp")
def run_prototype() -> None:
    "Run a python file named prototype.py everytime something changes."
    run(("cargo", "watch", "--clear", "--shell", "python3 prototype.py"))


def main() -> None:
    environ["RUST_BACKTRACE"] = "1"
    dispatch_commands(
        (
            start_solve,
            set_baseline,
            compare,
            criterion,
            iai,
            watch_run,
            do_run,
            run_release,
            run_prototype,
        ),
    )


if __name__ == "__main__":
    main()
