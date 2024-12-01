use std::collections::VecDeque;
use std::iter;

use hashbrown::HashMap;
use itertools::Itertools;
use num::integer::lcm;
use rayon::prelude::*;

use crate::common::fetch_input;

pub fn bruteforce<'a>(
  mut posv: Vec<&'a &'a str>,
  route: &'a [bool],
  map: &'a HashMap<&'a str, (&'a str, &'a str)>,
) {
  println!("Running {} concurrent searches", posv.len());
  for (i, step) in route.iter().cycle().enumerate() {
    let at_z = (posv.iter())
      .enumerate()
      .filter(|(_, loc)| loc.ends_with('Z'))
      .collect_vec();
    if 1 < at_z.len() {
      println!(
        "On step {i}, ready routes are {}",
        at_z.iter().map(|(i, n)| format!("{i} at {n}")).join(", ")
      );
      if at_z.len() == posv.len() {
        println!("Search complete.");
        break;
      }
    }
    posv.iter_mut().for_each(|pos| {
      let (left, right) = map.get(*pos).unwrap();
      *pos = if *step { right } else { left };
    });
  }
}

pub fn find_cycle<'a>(
  mut pos: &'a str,
  route: &'a [bool],
  map: &'a HashMap<&'a str, (&'a str, &'a str)>,
) -> (Vec<bool>, Vec<bool>) {
  let mut history = Vec::<&str>::new();
  let mut steps = route.iter().cycle();
  loop {
    let step = steps.next().unwrap();
    let mut candidates =
      (1..).map(|n| n * route.len()).take_while(|n| *n < history.len());
    match candidates.find(|n| history[history.len() - n] == pos) {
      Some(prev) => {
        let (prefix, cycle) = history.split_at(history.len() - prev);
        return (
          prefix.iter().map(|n| n.ends_with('Z')).collect_vec(),
          cycle.iter().map(|n| n.ends_with('Z')).collect_vec(),
        );
      },
      None => {
        history.push(pos);
        let (left, right) = map.get(pos).unwrap();
        pos = if *step { right } else { left }
      },
    }
  }
}

pub fn normalize_cycle<T: PartialEq>(
  mut prefix: Vec<T>,
  cycle: Vec<T>,
) -> (Vec<T>, Vec<T>) {
  let mut cycle = VecDeque::from(cycle);
  while Some(cycle.back().unwrap()) == prefix.last() {
    prefix.pop();
    cycle.rotate_right(1);
  }
  let mut cycle: Vec<_> = cycle.into();
  for section in (2..=cycle.len() / 2).filter(|n| cycle.len() % n == 0) {
    if (cycle.chunks_exact(section)).all(|s| s == &cycle[..section]) {
      cycle.truncate(section);
      break;
    }
  }
  (prefix, cycle)
}

pub fn print_bool_vec(v: &[bool]) -> String {
  v.iter().map(|b| if *b { '1' } else { '0' }).join("")
}

pub fn day8() {
  let input = fetch_input(8);
  let (route, map) = input.split_once("\n\n").unwrap();
  let route = route.chars().map(|c| c == 'R').collect_vec();
  println!("Route len: {}", route.len());
  let map = (map.trim().split('\n'))
    .map(|node| {
      let (name, sides) = node.split_once(" = (").unwrap();
      (name, sides.strip_suffix(')').unwrap().split_once(", ").unwrap())
    })
    .collect::<HashMap<_, _>>();
  let posv = map.keys().filter(|n| n.ends_with('A')).collect_vec();
  let cycles = (posv.iter())
    .map(|v| find_cycle(v, &route, &map))
    .map(|(prefix, cycle)| normalize_cycle(prefix, cycle))
    .collect_vec();
  for (prefix, cyc) in cycles.iter() {
    println!("{}[{}]", print_bool_vec(prefix), print_bool_vec(cyc))
  }
  // A glance at the above values reveals that the answer is infact LCM from the
  // next line. A general solution to this problem is extremely difficult to
  // write and not needed to answer the question.
  println!(
    "Found all cycles: max prefix = {}, lengths = {:?} LCM = {}",
    cycles.iter().map(|(pref, _)| pref.len()).max().unwrap(),
    cycles.iter().map(|(_, cyc)| cyc.len()).collect_vec(),
    cycles.iter().fold(1, |acc, (_, c)| lcm(acc, c.len()))
  );
  // Part 1 can be solved by adapting bruteforce(vec!["AAA"])
}
