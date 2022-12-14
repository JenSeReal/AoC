enum Field {
  Start,
  End,
  Field,
}

fn successors((x, y): (usize, usize), grid: &[Vec<u8>]) -> Vec<((usize, usize), u32)> {
  let rows = grid.len();
  let cols = grid[0].len();
  let current_elevation = grid[y][x];
  let mut successors = vec![];

  if y > 0 {
    successors.push((x, y - 1))
  }

  if y < rows - 1 {
    successors.push((x, y + 1));
  }

  if x > 0 {
    successors.push((x - 1, y));
  }

  if x < cols - 1 {
    successors.push((x + 1, y));
  }

  successors
    .iter()
    .filter(|(x, y)| {
      let elevation = grid[*y][*x];
      (current_elevation..=current_elevation + 1).contains(&elevation)
    })
    .map(|successor| (*successor, 1u32))
    .collect::<Vec<_>>()
}

fn distance((x, y): &(usize, usize), end: &(usize, usize)) -> u32 {
  (x.abs_diff(end.0) + y.abs_diff(end.1)) as u32
}

fn parse(input: &str) -> ((usize, usize), (usize, usize), Vec<Vec<u8>>) {
  let cols = input.lines().next().unwrap().len();
  let rows = input.lines().count();
  let mut start = (0, 0);
  let mut end = (0, 0);
  let mut grid = vec![vec![0; cols]; rows];

  for (row, line) in input.lines().enumerate() {
    for (col, c) in line.chars().enumerate() {
      let elevation = match c {
        'S' => {
          start = (col, row);
          'a'
        }
        'E' => {
          end = (col, row);
          'z'
        }
        'a'..='z' => c,
        _ => unreachable!(),
      };

      let elevation = elevation as u8 - b'a';
      grid[row][col] = elevation;
    }
  }

  (start, end, grid)
}

fn possible_starts(grid: &[Vec<u8>]) -> Vec<(usize, usize)> {
  grid
    .iter()
    .enumerate()
    .fold(vec![], |mut possible_starts, (row_idx, row)| {
      row.iter().enumerate().for_each(|(col_idx, &item)| {
        if item == 0 {
          possible_starts.push((col_idx, row_idx))
        }
      });
      possible_starts
    })
}

pub(crate) fn part_1(input: &str) -> u32 {
  let (start, end, grid) = parse(input);

  pathfinding::prelude::astar(
    &start,
    |pos| successors(*pos, &grid),
    |pos| distance(pos, &end),
    |pos| pos == &end,
  )
  .unwrap()
  .1
}

pub(crate) fn part_2(input: &str) -> u32 {
  let (_, end, grid) = parse(input);
  let possible_starts = possible_starts(&grid);

  possible_starts
    .iter()
    .filter_map(|start| {
      pathfinding::prelude::astar(
        start,
        |coordinate| successors(*coordinate, &grid),
        |coordinate| distance(coordinate, &end),
        |coordinate| coordinate == &end,
      )
    })
    .map(|path| path.1)
    .min()
    .unwrap()
}

#[cfg(test)]
pub(crate) mod tests {
  use super::*;

  const TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

  #[test]
  fn test_solve_part_1() {
    let res = part_1(TEST_INPUT);
    assert_eq!(res, 31);
  }

  #[test]
  fn test_solve_part_2() {
    let res = part_2(TEST_INPUT);
    assert_eq!(res, 29)
  }
}
