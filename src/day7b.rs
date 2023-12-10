use std::cmp::Ordering;
use std::hash::Hash;

use itertools::Itertools;

use crate::common::fetch_input_lines;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Card(usize);
impl Card {
  const RANKS: &'static str = "J23456789TQKA";
  pub fn new(rank: char) -> Self { Self(Self::RANKS.find(rank).unwrap()) }
  pub fn is_joker(&self) -> bool { self.0 == 0 }
}
#[derive(Debug, Clone, Eq)]
struct Hand {
  cards: [Card; 5],
  bid: usize,
}
impl Hand {
  pub fn kind(&self) -> usize {
    if self.cards.iter().all(|c| c.is_joker()) {
      return 6;
    }
    let jc = self.cards.iter().filter(|c| c.is_joker()).count();
    let mut counts = (self.cards.iter())
      .filter(|c| !c.is_joker())
      .counts()
      .into_values()
      .collect_vec();
    counts.sort_unstable_by(|a, b| b.cmp(a));
    counts[0] += jc;
    match &counts[..] {
      [5] => 6,
      [4, 1] => 5,
      [3, 2] => 4,
      [3, 1, 1] => 3,
      [2, 2, 1] => 2,
      [2, 1, 1, 1] => 1,
      [1, 1, 1, 1, 1] => 0,
      _ => unreachable!("Impossible counts for 5 elements: {counts:?}"),
    }
  }
}
impl PartialEq for Hand {
  fn eq(&self, other: &Self) -> bool { self.cards == other.cards }
}
impl Hash for Hand {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.cards.hash(state) }
}
impl Ord for Hand {
  fn cmp(&self, other: &Self) -> Ordering {
    self.kind().cmp(&other.kind()).then_with(|| self.cards.cmp(&other.cards))
  }
}
impl PartialOrd for Hand {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

fn parse_hand(line: &str) -> Hand {
  let (cards, bid) = line.split_once(' ').unwrap();

  let cardv = cards.chars().map(Card::new).collect_vec();
  Hand { cards: cardv.try_into().unwrap(), bid: bid.parse().unwrap() }
}

pub fn day7b() {
  let mut hands = fetch_input_lines(7).map(|s| parse_hand(&s)).collect_vec();
  hands.sort_unstable();
  let winnings =
    hands.iter().enumerate().map(|(i, hand)| (i + 1) * hand.bid).sum::<usize>();
  println!("Winnings: {winnings}");
}
