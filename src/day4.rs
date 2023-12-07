use std::collections::VecDeque;

use hashbrown::HashSet;

use crate::common::fetch_input_lines;

struct Game {
  id: usize,
  winners: HashSet<usize>,
  tries: Vec<usize>,
}
impl Game {
  pub fn wins(&self) -> usize {
    self.tries.iter().filter(|guess| self.winners.contains(*guess)).count()
  }
}

pub fn parse_card(card: &str) -> Game {
  let (head, data) = card.split_once(':').unwrap();
  let (winners, attempts) = data.split_once('|').unwrap();
  Game {
    id: head.strip_prefix("Card").unwrap().trim().parse().unwrap(),
    winners: (winners.split(' '))
      .map(|l| l.trim())
      .filter(|s| !s.is_empty())
      .map(|s| s.parse().unwrap())
      .collect(),
    tries: (attempts.split(' '))
      .map(|l| l.trim())
      .filter(|s| !s.is_empty())
      .map(|s| s.parse().unwrap())
      .collect(),
  }
}

pub fn day4() {
  let games = fetch_input_lines(4).map(|s| parse_card(&s)).collect::<Vec<_>>();
  let points = (games.iter())
    .map(|game| {
      game.wins().checked_sub(1).map_or(0usize, |c| 2usize.pow(c as u32))
    })
    .sum::<usize>();
  println!("Total points: {points}");
  let cards_count = games
    .iter()
    .scan(VecDeque::new(), |mults, game| {
      let own_mult = 1 + mults.pop_front().unwrap_or(0);
      let trickle = game.wins();
      mults.iter_mut().take(trickle).for_each(|mult| *mult += own_mult);
      if let Some(extra) = trickle.checked_sub(mults.len()) {
        for _ in 0..extra {
          mults.push_back(own_mult);
        }
      }
      Some(own_mult)
    })
    .sum::<usize>();
  println!("Total cards per p2: {cards_count}")
}
