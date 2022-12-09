fn solve_with_marker(input: &str, marker: usize) -> usize {
  input
    .as_bytes()
    .windows(marker)
    .position(|window| {
      window
        .iter()
        .enumerate()
        .all(|(idx, c)| !window[..idx].contains(c))
    })
    .unwrap()
    + marker
}

pub(crate) fn part_1(input: &str) -> usize {
  solve_with_marker(input, 4)
}

pub(crate) fn part_2(input: &str) -> usize {
  solve_with_marker(input, 14)
}

#[cfg(test)]
pub(crate) mod tests {
  use super::*;

  #[test]
  fn test_solve_part_1() {
    assert_eq!(part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    assert_eq!(part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    assert_eq!(part_1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    assert_eq!(part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    assert_eq!(part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
  }

  #[test]
  fn test_solve_part_2() {
    assert_eq!(part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    assert_eq!(part_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    assert_eq!(part_2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    assert_eq!(part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    assert_eq!(part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
  }
}
