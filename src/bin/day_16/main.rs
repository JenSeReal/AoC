use itertools::Itertools;
use nom::{
  branch::alt,
  bytes::complete::{tag, take},
  character::complete::{self, line_ending},
  multi::separated_list1,
  sequence::{preceded, tuple},
  IResult,
};
use std::{
  cmp::Reverse,
  collections::{BTreeMap, HashMap},
};

const INPUT: &str = include_str!("input");

#[derive(Debug, Clone)]
struct Valve<'a> {
  flow_rate: u8,
  outlets: Vec<&'a str>,
}

type FlowRates = Vec<u8>;
type DistancesMatrix = Vec<Vec<u8>>;

fn floyd_warshall(valves: &BTreeMap<&str, Valve>) -> DistancesMatrix {
  let valve_index_lookup: HashMap<&str, _> = valves
    .iter()
    .enumerate()
    .map(|(i, (id, _))| (*id, i))
    .collect();

  let mut dist = vec![vec![u8::MAX; valves.len()]; valves.len()];
  for (i, (_, valves)) in valves.iter().enumerate() {
    for outlet in valves.outlets.iter() {
      let j = valve_index_lookup[&outlet[..]];
      dist[i][j] = 1;
    }
  }
  (0..dist.len()).for_each(|i| {
    dist[i][i] = 0;
  });
  for k in 0..dist.len() {
    for i in 0..dist.len() {
      for j in 0..dist.len() {
        let (result, overflow) = dist[i][k].overflowing_add(dist[k][j]);
        if !overflow && dist[i][j] > result {
          dist[i][j] = result;
        }
      }
    }
  }
  dist
}

fn parse_valve(input: &str) -> IResult<&str, (&str, Valve)> {
  let (input, (id, flow_rate, outlets)) = tuple((
    preceded(tag("Valve "), take(2usize)),
    preceded(tag(" has flow rate="), complete::u8),
    preceded(
      alt((
        tag("; tunnels lead to valves "),
        tag("; tunnel leads to valve "),
      )),
      separated_list1(tag(", "), take(2usize)),
    ),
  ))(input)?;
  Ok((input, (id, Valve { flow_rate, outlets })))
}

fn parse(input: &str) -> (FlowRates, DistancesMatrix) {
  let valves = separated_list1(line_ending, parse_valve)(input)
    .unwrap()
    .1
    .into_iter()
    .collect::<BTreeMap<&str, Valve>>();

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

  let distances_matrix = interesting_valves
    .iter()
    .map(|&i| {
      interesting_valves
        .iter()
        .map(|&j| distances_matrix[i][j])
        .collect()
    })
    .collect();

  (flow_rates, distances_matrix)
}

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

fn branch_and_bound(
  flow_rates: &FlowRates,
  shortest_path_lengths: &DistancesMatrix,
  state: State,
  best: &mut u16,
) {
  if let Some(solution) = state.solution() {
    *best = solution.max(*best);
    return;
  }
  let bound_branch_pairs = state
    .branch(flow_rates, shortest_path_lengths)
    .into_iter()
    .map(|state| (state.bound(flow_rates), state))
    .filter(|(bound, _)| bound > best)
    .sorted_unstable_by_key(|(bound, _)| Reverse(*bound))
    .collect_vec();
  for (bound, branch) in bound_branch_pairs {
    if bound > *best {
      branch_and_bound(flow_rates, shortest_path_lengths, branch, best);
    }
  }
}

fn part_1((flow_rates, distances_matrix): &(FlowRates, DistancesMatrix)) -> u16 {
  let mut best = 0;
  branch_and_bound(
    flow_rates,
    distances_matrix,
    State {
      visitors: [
        VisitorState { next: 0, eta: 0 },
        VisitorState {
          next: 0,
          eta: u8::MAX,
        },
      ],
      visited: 0,
      pressure_released: 0,
      current_flow: 0,
      minutes_remaining: 30,
    },
    &mut best,
  );
  best
}

fn part_2((flow_rates, distances_matrix): &(FlowRates, DistancesMatrix)) -> u16 {
  let mut best = 0;
  branch_and_bound(
    flow_rates,
    distances_matrix,
    State {
      visitors: [
        VisitorState { next: 0, eta: 0 },
        VisitorState { next: 0, eta: 0 },
      ],
      visited: 0,
      pressure_released: 0,
      current_flow: 0,
      minutes_remaining: 26,
    },
    &mut best,
  );
  best
}

#[cfg(test)]
mod tests {
  use super::*;
  const TEST_INPUT: &str = include_str!("sample");

  #[test]
  fn test_solve_part_1() {
    assert_eq!(part_1(&parse(TEST_INPUT)), 1651);
  }

  #[test]
  fn test_solve_part_2() {
    assert_eq!(part_2(&parse(TEST_INPUT)), 1707);
  }
}

fn main() {
  let parsed = parse(INPUT);
  println!("Part 1: {}", part_1(&parsed));
  println!("Part 2: {}", part_2(&parsed));
}
