use std::{cmp::Ordering, fs::read_to_string};

use nom::{
  branch::alt,
  bytes::complete::tag,
  character::complete::{self, newline},
  combinator::{all_consuming, map},
  multi::{separated_list0, separated_list1},
  sequence::{delimited, separated_pair},
  IResult,
};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
enum Packet {
  Num(u32),
  List(Vec<Packet>),
}

impl Ord for Packet {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    match (self, other) {
      (Self::List(a), Self::List(b)) => a.cmp(b),
      (Self::List(a), Self::Num(b)) => a.cmp(&vec![Self::Num(*b)]),
      (Self::Num(a), Self::List(b)) => vec![Self::Num(*a)].cmp(&b),
      (Self::Num(a), Self::Num(b)) => a.cmp(b),
    }
  }
}

impl PartialOrd for Packet {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
  alt((
    map(
      delimited(tag("["), separated_list0(tag(","), parse_packet), tag("]")),
      Packet::List,
    ),
    map(complete::u32, Packet::Num),
  ))(input)
}

fn parse_pair(input: &str) -> IResult<&str, (Packet, Packet)> {
  separated_pair(parse_packet, newline, parse_packet)(input)
}

fn parse(input: &str) -> Vec<(Packet, Packet)> {
  all_consuming(separated_list1(tag("\n\n"), parse_pair))(input)
    .unwrap()
    .1
}

pub(crate) fn part_1(input: &str) -> usize {
  let pairs = parse(input);

  pairs
    .iter()
    .enumerate()
    .filter_map(|(idx, (left, right))| match left < right {
      true => Some(idx + 1),
      false => None,
    })
    .sum()
}

pub(crate) fn part_2(input: &str) -> usize {
  let pairs = parse(input);
  let mut packets = pairs
    .iter()
    .map(|(left, right)| [left, right])
    .flatten()
    .collect::<Vec<_>>();

  let ((_, divider_packet_2), (_, divider_packet_6)) = (
    parse_packet("[[2]]").unwrap(),
    parse_packet("[[6]]").unwrap(),
  );

  packets.extend([&divider_packet_2, &divider_packet_6]);
  packets.sort();

  packets
    .into_iter()
    .positions(|packet| packet == &divider_packet_2 || packet == &divider_packet_6)
    .map(|pos| pos + 1)
    .product()
}

fn main() {
  let input = read_to_string("assets/day_13").unwrap();
  let part_1 = part_1(&input);
  println!("Part 1: {}", part_1);
  let part_2 = part_2(&input);
  println!("Part 2: {}", part_2);
}

#[cfg(test)]
pub(crate) mod tests {
  use super::*;

  const TEST_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

  #[test]
  fn test_solve_part_1() {
    let res = part_1(TEST_INPUT);
    assert_eq!(res, 13);
  }

  #[test]
  fn test_solve_part_2() {
    let res = part_2(TEST_INPUT);
    assert_eq!(res, 140)
  }
}
