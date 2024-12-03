use hashbrown::HashMap;
use itertools::Itertools;

use crate::common::fetch_input_lines;

pub fn day11() {
  let mut rows = HashMap::<u128, u128>::new();
  let mut columns = HashMap::<u128, u128>::new();
  let mut galcnt = 0;
  for (rown, row) in
    fetch_input_lines(11).enumerate().map(|(i, d)| (i as u128, d))
  {
    for (coln, char) in row.chars().enumerate().map(|(i, d)| (i as u128, d)) {
      if char == '#' {
        galcnt += 1;
        (rows.raw_entry_mut().from_key(&rown))
          .and_modify(|_, v| *v += 1)
          .or_insert(rown, 1);
        (columns.raw_entry_mut().from_key(&coln))
          .and_modify(|_, v| *v += 1)
          .or_insert(coln, 1);
      }
    }
  }
  let process_dimension = |dim: &HashMap<u128, u128>, gap: u128| -> u128 {
    let mut vec = dim.iter().map(|(a, b)| (*a, *b)).collect_vec();
    vec.sort_unstable_by_key(|(pos, _)| *pos);
    let mut behind = 0;
    let mut total_steps = 0;
    for ((lpos, lcnt), (hpos, _)) in vec.into_iter().tuple_windows() {
      behind += lcnt;
      total_steps += ((hpos - lpos - 1) * gap + 1) * behind * (galcnt - behind);
    }
    total_steps
  };
  let total_a = process_dimension(&rows, 2) + process_dimension(&columns, 2);
  let total_b =
    process_dimension(&rows, 1000000) + process_dimension(&columns, 1000000);
  println!(
    "Total distance with expansion of 2: {total_a} and with 1000000: {total_b}"
  );
}
