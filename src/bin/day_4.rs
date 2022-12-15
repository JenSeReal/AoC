use std::fs::read_to_string;

fn parse(input: &str) -> impl Iterator<Item = ((u32, u32), (u32, u32))> + '_ {
  input
    .lines()
    .map(|pair| pair.split_once(",").unwrap())
    .map(|(one, two)| (one.split_once("-").unwrap(), two.split_once("-").unwrap()))
    .map(|(one, two)| {
      (
        (one.0.parse::<u32>().unwrap(), one.1.parse::<u32>().unwrap()),
        (two.0.parse::<u32>().unwrap(), two.1.parse::<u32>().unwrap()),
      )
    })
}

pub(crate) fn part_1(input: &str) -> u32 {
  parse(input).fold(0u32, |acc, ((one_min, one_max), (two_min, two_max))| {
    if one_min <= two_min && one_max >= two_max || two_min <= one_min && two_max >= one_max {
      acc + 1
    } else {
      acc
    }
  })
}

pub(crate) fn part_2(input: &str) -> u32 {
  parse(input).fold(0, |acc, ((one_min, one_max), (two_min, two_max))| {
    let range_one = one_min..=one_max;
    let range_two = two_min..=two_max;

    if range_one.contains(&two_min) || range_two.contains(&one_min) {
      acc + 1
    } else {
      acc
    }
  })
}

fn main() {
  let input = read_to_string("assets/day_4").unwrap();
  let part_1 = part_1(&input);
  println!("Part 1: {}", part_1);
  let part_2 = part_2(&input);
  println!("Part 2: {}", part_2);
}

#[cfg(test)]
pub(crate) mod tests {
  use super::*;

  const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

  #[test]
  fn test_solve_part_1() {
    let res = part_1(TEST_INPUT);
    assert_eq!(res, 2);
  }

  #[test]
  fn test_solve_part_2() {
    let res = part_2(TEST_INPUT);
    assert_eq!(res, 4)
  }
}
