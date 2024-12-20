use std::mem;

use hashbrown::HashSet;
use itertools::{Either, Itertools};

use crate::common::fetch_input_lines;

#[derive(Clone)]
enum Act {
  Fail,
  Ban(Vec<u8>),
  Pass
}

pub type State = [Act; 100];

pub fn day5() {
  let (rules, seqs): (Vec<(u8, u8)>, Vec<Vec<u8>>) = fetch_input_lines(5)
    .filter(|p| !p.trim().is_empty())
    .partition_map(|p| if p.contains('|') {
      Either::Left(p.split('|').map(|n| n.trim().parse::<u8>().unwrap()).collect_tuple::<(u8, u8)>().unwrap())
    } else {
      Either::Right(p.split(',').map(|n| n.trim().parse::<u8>().unwrap()).collect_vec())
    });
  let mut state: State = [const { Act::Pass }; 100];
  for rule in &rules {
    let ent = &mut state[rule.1 as usize];
    if let Act::Ban(v) = ent {
      v.push(rule.0)
    } else {
      *ent = Act::Ban(vec![rule.0])
    }
  }
  let mut midpagesum = 0u64;
  let mut bad_seqs = Vec::new();
  'seqs:for seq in seqs {
    let mut tstate = state.clone();
    for num in &seq {
      let mut prev_state = Act::Pass;
      mem::swap(&mut prev_state, &mut tstate[*num as usize]);
      match prev_state {
        Act::Pass => (),
        Act::Fail => {
          bad_seqs.push(seq);
          continue 'seqs
        },
        Act::Ban(nums) => for num in nums {
          tstate[num as usize] = Act::Fail;
        },
      }
    }
    midpagesum += seq[seq.len() / 2] as u64;
  }
  let mut bad_midpagesum = 0u64;
  for seq in bad_seqs {
    // find the rules that apply
    let mut sub_rules = rules.iter().filter(|r| seq.contains(&r.0) && seq.contains(&r.1)).collect_vec();
    // toposort!
    let mut sorted_vec = Vec::new();
    let mut all_elements = seq.into_iter().collect::<HashSet<u8>>();
    while !all_elements.is_empty() {
      let minimal = *all_elements.iter().find(|i| !sub_rules.iter().any(|r| r.1 == **i)).expect("Circular order!");
      sorted_vec.push(minimal);
      all_elements.remove(&minimal);
      if !sub_rules.is_empty() {
        let prev_size = sub_rules.len();
        sub_rules.retain(|r| r.0 != minimal);
        assert_ne!(prev_size, sub_rules.len(), "{minimal} outside partial order defined by {all_elements:?}");
      }
    }
    bad_midpagesum += sorted_vec[sorted_vec.len() / 2] as u64;
  }
  eprintln!("The mid-page sum of the valid sequences is {midpagesum}. For the fixed ones, it's {bad_midpagesum}")
}