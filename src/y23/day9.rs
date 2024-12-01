use itertools::Itertools;

use crate::common::fetch_input_lines;

fn extrapolate(seq: &[i64]) -> i64 {
  let deltas = seq.iter().tuple_windows().map(|(a, b)| b - a).collect_vec();
  seq.last().unwrap() + if seq.iter().all_equal() { 0 } else { extrapolate(&deltas) }
}

pub fn day9() {
  let result = fetch_input_lines(9)
    .map(|l| {
      let mut numbers = l.split(' ').map(|n| n.parse().unwrap()).collect_vec();
      numbers.reverse();
      extrapolate(&numbers)
    })
    .sum::<i64>();
  println!("Result: {result}")
}
