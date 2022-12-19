#![allow(dead_code)]
#![allow(unused)]

use std::{
  cmp::Reverse,
  collections::{BTreeMap, HashMap, HashSet},
  fs::read_to_string,
  hash::Hash,
};

use itertools::Itertools;
use nom::{
  branch::alt,
  bytes::complete::{is_not, tag, take, take_while},
  character::{
    complete::{self, alpha1, line_ending, multispace0},
    is_alphabetic,
  },
  combinator::{all_consuming, map},
  multi::{count, separated_list1},
  sequence::{preceded, tuple},
  Finish, IResult, Parser,
};

const DAY: usize = 16;
const REMAINING_MINUTES: usize = 30;
const TIME_TO_OPEN_VALVE: usize = 1;
const TIME_TO_TRAVEL_BETWEEN_VALVES: usize = 1;

#[derive(Debug, Clone)]
enum Status {
  Closed,
  Opened,
}

#[derive(Debug, Clone)]
struct Valve {
  flow_rate: u8,
  outlets: Vec<String>,
  state: Status,
}

type FlowRates = Vec<u8>;
type DistancesMatrix = Vec<Vec<u8>>;

#[derive(Default, Debug, Clone, Copy)]
struct VisitorState {
  next: u8,
  eta: u8,
}

#[derive(Default, Debug, Clone, Copy)]
struct State {
  visitors: [VisitorState; 2],
  visited: u16,
  pressure_released: u16,
  current_flow: u16,
  minutes_remaining: u8,
}

impl State {
  fn visited(&self, i: usize) -> bool {
    self.visited & (1 << i) != 0
  }

  fn solution(&self) -> Option<u16> {
    (!self.visitors.iter().any(|v| v.eta < self.minutes_remaining))
      .then_some(self.pressure_released + (self.minutes_remaining as u16 + 1) * self.current_flow)
  }

  fn bound(mut self, flow_rates: &FlowRates) -> u16 {
    let mut remaining_flow_rate_indices = (0..flow_rates.len())
      .filter(|&i| !self.visited(i) && !self.visitors.iter().any(|v| v.next == i as u8))
      .collect_vec();
    remaining_flow_rate_indices.sort_unstable_by_key(|&i| flow_rates[i]);
    while self.minutes_remaining > 0 {
      self.minutes_remaining -= 1;
      self.pressure_released += self.current_flow;
      for visitor in self.visitors.iter_mut() {
        if visitor.eta > 0 {
          visitor.eta -= 1;
          continue;
        }
        self.current_flow += flow_rates[visitor.next as usize] as u16;
        if let Some(i) = remaining_flow_rate_indices.pop() {
          visitor.next = i as u8;
          visitor.eta = 1;
        } else {
          visitor.eta = u8::MAX;
        }
      }
    }
    self.pressure_released + self.current_flow
  }

  fn branch(
    mut self,
    flow_rates: &FlowRates,
    shortest_path_lengths: &DistancesMatrix,
  ) -> impl IntoIterator<Item = Self> {
    self.pressure_released += self.current_flow;
    self.minutes_remaining -= 1;
    let mut branches = vec![self];
    for (visitor_idx, visitor) in self.visitors.into_iter().enumerate() {
      if visitor.eta > 0 {
        branches
          .iter_mut()
          .for_each(|state| state.visitors[visitor_idx].eta -= 1);
        continue;
      }
      branches.iter_mut().for_each(|state| {
        state.visited |= 1 << visitor.next;
        state.current_flow += flow_rates[visitor.next as usize] as u16;
      });
      branches = branches
        .iter()
        .flat_map(|&state| {
          shortest_path_lengths[visitor.next as usize]
            .iter()
            .enumerate()
            .filter(move |&(destination, _)| {
              !state.visited(destination)
                && !state.visitors.iter().any(|v| v.next == destination as u8)
            })
            .map(move |(destination, &distance)| {
              let mut next_state = state;
              next_state.visitors[visitor_idx].next = destination as u8;
              next_state.visitors[visitor_idx].eta = distance;
              next_state
            })
        })
        .chain([{
          let mut state = branches[0];
          state.visitors[visitor_idx].eta = u8::MAX;
          state
        }])
        .collect();
    }
    branches
  }
}

fn parse_valve(input: &str) -> IResult<&str, (&str, Valve)> {
  let (input, id) = preceded(tag("Valve "), take(2usize))(input)?;
  let (input, flow_rate) = preceded(tag(" has flow rate="), complete::u8)(input)?;
  let (input, to_valves) = preceded(
    alt((
      tag("; tunnels lead to valves "),
      tag("; tunnel leads to valve "),
    )),
    separated_list1(tag(", "), map(alpha1, Into::into)),
  )(input)?;
  Ok((
    input,
    (
      id,
      Valve {
        flow_rate,
        outlets: to_valves,
        state: Status::Closed,
      },
    ),
  ))
}

fn parse(input: &str) -> (BTreeMap<&str, Valve>, FlowRates, DistancesMatrix) {
  let valves = input
    .trim()
    .lines()
    .fold(BTreeMap::new(), |mut valves, line| {
      let (id, valve) = all_consuming(parse_valve)(line).finish().unwrap().1;
      valves.insert(id, valve);
      valves
    });

  let distances_matrix = floyd_warshall(&valves);
  let interesting_valves = valves
    .iter()
    .enumerate()
    .filter(|(_, (&id, valve))| id == "AA" || valve.flow_rate > 0)
    .map(|(i, _)| i)
    .collect_vec();

  let flow_rates = valves
    .iter()
    .enumerate()
    .filter(|(i, (_, _))| interesting_valves.contains(i))
    .map(|(_, (_, valve))| valve.flow_rate)
    .collect_vec();

  let compressed = interesting_valves
    .iter()
    .map(|&i| {
      interesting_valves
        .iter()
        .map(|&j| distances_matrix[i][j])
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  (valves, flow_rates, compressed)
}

fn floyd_warshall(valves: &BTreeMap<&str, Valve>) -> DistancesMatrix {
  let index_lookup = valves
    .iter()
    .enumerate()
    .map(|(i, (id, _))| (*id, i))
    .collect::<BTreeMap<&str, _>>();

  let mut distances_matrix = vec![vec![u8::MAX; valves.len()]; valves.len()];

  for (i, (id, valve)) in valves.iter().enumerate() {
    for outlet in valve.outlets.iter() {
      let j = index_lookup[&outlet[..]];
      distances_matrix[i][j] = 1;
    }
  }

  (0..distances_matrix.len()).for_each(|i| {
    distances_matrix[i][i] = 0;
  });

  for k in 0..distances_matrix.len() {
    for i in 0..distances_matrix.len() {
      for j in 0..distances_matrix.len() {
        let (result, is_overflow) = distances_matrix[i][k].overflowing_add(distances_matrix[k][j]);
        if !is_overflow && distances_matrix[i][j] > result {
          distances_matrix[i][j] = result;
        }
      }
    }
  }
  distances_matrix
}

fn branch_and_bound(
  flow_rates: &FlowRates,
  distances_matrix: &DistancesMatrix,
  state: State,
  best: &mut u16,
) {
  if let Some(solution) = state.solution() {
    *best = solution.max(*best);
    return;
  }
  let bound_branch_pairs = state
    .branch(flow_rates, distances_matrix)
    .into_iter()
    .map(|state| (state.bound(flow_rates), state))
    .filter(|(bound, _)| bound > best)
    .sorted_unstable_by_key(|(bound, _)| Reverse(*bound))
    .collect_vec();
  for (bound, branch) in bound_branch_pairs {
    if bound > *best {
      branch_and_bound(flow_rates, distances_matrix, branch, best);
    }
  }
}

pub(crate) fn part_1(input: &str) -> u16 {
  let (valves, flow_rates, distances_matrix) = parse(input);

  let initial_state = State {
    visitors: [
      VisitorState {
        next: 0,
        eta: valves.iter().next().unwrap().1.flow_rate,
      },
      VisitorState {
        next: 0,
        eta: u8::MAX,
      },
    ],
    visited: 0,
    pressure_released: 0,
    current_flow: 0,
    minutes_remaining: REMAINING_MINUTES.try_into().unwrap(),
  };

  let mut best = 0;
  branch_and_bound(&flow_rates, &distances_matrix, initial_state, &mut best);

  best
}

#[allow(unused)]
pub(crate) fn part_2(input: &str) -> u16 {
  let (valves, flow_rates, distances_matrix) = parse(input);

  let initial_state = State {
    visitors: [
      VisitorState {
        next: 0,
        eta: valves.iter().next().unwrap().1.flow_rate,
      },
      VisitorState {
        next: 0,
        eta: u8::MAX,
      },
    ],
    visited: 0,
    pressure_released: 0,
    current_flow: 0,
    minutes_remaining: 26,
  };

  let mut best = 0;
  branch_and_bound(&flow_rates, &distances_matrix, initial_state, &mut best);

  best
}

fn main() {
  let input = read_to_string(format!("assets/day_{}", DAY)).unwrap();
  println!("Part 1: {}", part_1(&input));
  println!("Part 2: {}", part_2(&input));
}

#[cfg(test)]
pub(crate) mod tests {
  use super::*;

  const TEST_INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

  #[test]
  fn test_solve_part_1() {
    let res = part_1(TEST_INPUT);
    assert_eq!(res, 1651);
  }

  #[test]
  fn test_solve_part_2() {
    let res = part_2(TEST_INPUT);
    assert_eq!(res, 1707)
  }
}
