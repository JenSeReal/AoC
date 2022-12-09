use std::collections::HashSet;

fn parse_part_1(input: &str) -> impl Iterator<Item = (HashSet<char>, HashSet<char>)> + '_ {
  input
    .lines()
    .map(|rucksack| rucksack.split_at(rucksack.len() / 2))
    .map(|(left_compartment, right_compartment)| {
      (
        left_compartment.chars().collect::<HashSet<char>>(),
        right_compartment.chars().collect::<HashSet<char>>(),
      )
    })
}

fn priority(c: char) -> u32 {
  match c.is_uppercase() {
    true => c as u32 - 38,
    false => c as u32 - 96,
  }
}

pub(crate) fn part_1(input: &str) -> u32 {
  let parsed = parse_part_1(input);

  parsed
    .map(|(left_compartment, right_compartment)| {
      left_compartment
        .intersection(&right_compartment)
        .cloned()
        .collect::<Vec<_>>()
    })
    .flatten()
    .map(priority)
    .sum::<u32>()
}

pub(crate) fn part_2(input: &str) -> u32 {
  let parsed = input.lines().collect::<Vec<_>>();
  let parsed = parsed.chunks_exact(3);
  parsed
    .map(|group| {
      group[0]
        .chars()
        .find(|item| group[1].contains(*item) && group[2].contains(*item))
        .unwrap()
    })
    .map(priority)
    .sum::<u32>()
}

#[cfg(test)]
pub(crate) mod tests {
  use super::*;

  const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

  #[test]
  fn test_solve_part_1() {
    let res = part_1(TEST_INPUT);
    assert_eq!(res, 157);
  }

  #[test]
  fn test_solve_part_2() {
    let res = part_2(TEST_INPUT);
    assert_eq!(res, 70)
  }
}
