fn parse(input: &str) -> impl Iterator<Item = u32> + '_ {
  input.split("\n\n").map(|inventory| {
    inventory
      .lines()
      .map(|item| item.parse::<u32>().unwrap())
      .sum::<u32>()
  })
}

pub(crate) fn part_1(input: &str) -> u32 {
  parse(input).max().unwrap()
}

pub(crate) fn part_2(input: &str) -> u32 {
  let mut inventories = parse(input).collect::<Vec<_>>();

  inventories.sort_by(|a, b| b.cmp(a));
  inventories.iter().take(3).sum()
}

#[cfg(test)]
pub(crate) mod tests {
  use super::*;

  const TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

  #[test]
  fn test_solve_part_1() {
    let res = part_1(TEST_INPUT);
    assert_eq!(res, 24000);
  }

  #[test]
  fn test_solve_part_2() {
    let res = part_2(TEST_INPUT);
    assert_eq!(res, 45000)
  }
}
