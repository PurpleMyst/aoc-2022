use std::fmt::Display;

fn to_base5(n: u64) -> Vec<i8> {
    let mut n = n;
    let mut result = Vec::new();

    while n != 0 {
        result.push((n % 5) as i8);
        n /= 5;
    }

    result.reverse();

    result
}

fn to_snafu(n: u64) -> String {
    let mut b5 = to_base5(n);
    for i in (0..b5.len()).rev() {
        match b5[i] {
            0 | 1 | 2 => {}
            n => {
                if i == 0 {
                    b5.insert(0, 1);
                    b5[1] = n - 5;
                } else {
                    b5[i - 1] += 1;
                    b5[i] = n - 5;
                }
            }
        }
    }

    b5.into_iter()
        .map(|d| match d {
            0 => '0',
            1 => '1',
            2 => '2',
            -1 => '-',
            -2 => '=',
            _ => unreachable!(),
        })
        .collect()
}

fn parse_snafu(s: &str) -> u64 {
    s.bytes().fold(0u64, |acc, b| match b {
        b'0' => acc * 5,
        b'1' => acc * 5 + 1,
        b'2' => acc * 5 + 2,
        b'-' => acc * 5 - 1,
        b'=' => acc * 5 - 2,
        _ => unreachable!(),
    })
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let part1 = include_str!("input.txt").lines().map(parse_snafu).sum();
    (to_snafu(part1), "Merry Christmas!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snafus() {
        assert_eq!(to_snafu(1), "1");
        assert_eq!(to_snafu(2), "2");
        assert_eq!(to_snafu(3), "1=");
        assert_eq!(to_snafu(4), "1-");
        assert_eq!(to_snafu(5), "10");
        assert_eq!(to_snafu(6), "11");
        assert_eq!(to_snafu(7), "12");
        assert_eq!(to_snafu(8), "2=");
        assert_eq!(to_snafu(9), "2-");
        assert_eq!(to_snafu(10), "20");
        assert_eq!(to_snafu(15), "1=0");
        assert_eq!(to_snafu(20), "1-0");
        assert_eq!(to_snafu(2022), "1=11-2");
        assert_eq!(to_snafu(12345), "1-0---0");
        assert_eq!(to_snafu(314159265), "1121-1110-1=0");
    }
}
