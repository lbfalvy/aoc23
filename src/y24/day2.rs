use itertools::Itertools;

use crate::common::fetch_input_lines;

pub fn safe_bar_one(seq: &[i32], mut chk: impl FnMut(i32, i32) -> bool) -> bool {
  for i in 0..seq.len() - 2 {
    if !chk(seq[i], seq[i + 1]) {
      let skip_this = (i == 0 || chk(seq[i - 1], seq[i + 1]))
        && seq[i + 1..].iter().tuple_windows().all(|(a, b)| chk(*a, *b));
      let skip_next = i + 2 == seq.len()
        || chk(seq[i], seq[i + 2])
          && seq[i + 2..].iter().tuple_windows().all(|(a, b)| chk(*a, *b));
      return skip_this || skip_next;
    }
  }
  return true;
}

pub fn day2() {
  let reports = fetch_input_lines(2)
    .map(|l| l.split(' ').map(|s| s.trim().parse::<i32>().unwrap()).collect_vec())
    .collect_vec();
  let safe = reports.iter().filter(|v| {
    let deltas = v.iter().tuple_windows().map(|(a, b)| a - b).collect_vec();
    deltas.iter().all(|d| 0 < *d && *d < 4) || deltas.iter().all(|d| -4 < *d && *d < 0)
  }).count();
  let safe_with_dampener = reports.iter().filter(|v| {
    safe_bar_one(&v, |a, b| 0 < a - b && a - b < 4)
    || safe_bar_one(&v, |a, b| 0 < b - a && b - a < 4)
  }).count();
  println!("Safe: {safe}, Safe with dampener: {safe_with_dampener}")
}