use itertools::Itertools;

use crate::common::{fetch_input, fetch_input_lines};

static HEADINGS: &[char] = &['^', '>', 'v', '<'];

fn next_hdg(hdg: char) -> char {
  HEADINGS[(HEADINGS.iter().find_position(|c| **c == hdg).unwrap().0 + 1) % 4]
}
fn next_pos(hdg: char, x: i32, y: i32) -> (i32, i32) {
  match hdg {
    '>' => (x + 1, y),
    '<' => (x - 1, y),
    '^' => (x, y - 1),
    'v' => (x, y + 1),
    _ => panic!("Not a heading {hdg:?}")
  }
}

pub fn day6b() {
  let mut data = fetch_input_lines(6)
    .map(|l| l.chars().collect_vec())
    .collect_vec();
  let (mut hdg, mut x, mut y) = (data.iter().enumerate())
    .find_map(|(y, l)| 
      (l.iter().enumerate())
      .find_map(|(x, c)| HEADINGS.contains(c).then_some((*c, x as i32, y as i32))))
    .expect("Guard not found!");
  eprintln!("Starting from {x};{y}, limits are {};{}", data[0].len(), data.len());
  let mut bounces = 0;
  loop {
    let (x2, y2) = next_pos(hdg, x, y);
    if 0 > x2 || 0 > y2 || y2 >= data.len() as i32 || x2 >= data[0].len() as i32 {
      data[y as usize][x as usize] = hdg;
      break;
    }
    if data[y as usize][x as usize] == next_hdg(hdg) &&
      data[y2 as usize][x2 as usize] == '.' {
        bounces += 1;
    }
    data[y as usize][x as usize] = hdg;
    if data[y2 as usize][x2 as usize] == '#' {
      eprintln!("Bounced at {x};{y}{hdg}");
      hdg = next_hdg(hdg);
      continue;
    }
    x = x2;
    y = y2;
  }
  let all_visited: usize = data.iter().map(|l| l.iter().filter(|c| HEADINGS.contains(&c)).count()).sum();
  eprintln!("Total visited: {all_visited}, potential loop bounces: {bounces}")
}