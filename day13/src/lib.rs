use std::{fmt::Display, str::Chars};

#[derive(Debug, PartialEq, Clone)]
enum Item {
    Atom(u8),
    List(Vec<Item>),
}

impl Eq for Item {}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Item::Atom(l), Item::Atom(r)) => l.cmp(r),
            (Item::Atom(l), Item::List(r)) => [Item::Atom(*l)].as_slice().cmp(r.as_slice()),
            (Item::List(l), Item::Atom(r)) => l.as_slice().cmp([Item::Atom(*r)].as_slice()),
            (Item::List(l), Item::List(r)) => l.cmp(r),
        }
    }
}

fn parse_item(cs: &mut Chars) -> Item {
    if cs.as_str().starts_with('[') {
        cs.next().unwrap(); // consume the [
        let mut contents = Vec::with_capacity(8);
        while !cs.as_str().starts_with(']') {
            contents.push(parse_item(cs));
            if cs.as_str().starts_with(',') {
                cs.next().unwrap();
            }
        }
        cs.next().unwrap(); // consume the ]
        Item::List(contents)
    } else {
        let s = cs.as_str();
        let len = s.find([',', ']']).unwrap_or(s.len());
        let item = s[..len].parse().unwrap();
        cs.take(len).for_each(|_| ());
        Item::Atom(item)
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut packets: Vec<_> = include_str!("input.txt")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|s| parse_item(&mut s.chars()))
        .collect();

    let p1: usize = packets
        .chunks(2)
        .map(|pair| <_ as TryInto<&[Item; 2]>>::try_into(pair).unwrap())
        .enumerate()
        .filter(|(_, [l, r])| l < r)
        .map(|(i, _)| i + 1)
        .sum();

    let divider1 = Item::List(vec![Item::List(vec![Item::Atom(2)])]);
    let divider2 = Item::List(vec![Item::List(vec![Item::Atom(6)])]);

    packets.push(divider1.clone());
    packets.push(divider2.clone());
    packets.sort_unstable();
    let idx1 = packets.iter().position(|packet| packet == &divider1).unwrap();
    let idx2 = packets.iter().position(|packet| packet == &divider2).unwrap();

    (p1, (idx1 + 1) * (idx2 + 1))
}
