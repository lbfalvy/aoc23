use std::fmt::Debug;
use std::ops::{Add, Range, Sub};

use hashbrown::HashMap;
use itertools::Itertools;

use crate::common::fetch_input;

pub struct RangeMap<T> {
  ranges: Vec<(usize, Range<T>, T)>,
}
impl<T: Debug + Clone + Ord + Add<Output = T> + Sub<Output = T>> RangeMap<T> {
  pub fn new() -> Self { Self { ranges: Vec::new() } }
  pub fn add_remap(&mut self, id: usize, source: Range<T>, target: T) {
    if source.start == source.end {
      return;
    }
    let potential_index =
      (self.ranges.binary_search_by_key(&&source.start, |(_, r, _)| &r.start))
        .expect_err("Two ranges start at the same index");
    if potential_index < self.ranges.len() {
      let (_, next, _) = &self.ranges[potential_index];
      assert!(
        source.end <= next.start,
        "New range overlaps at the top {source:?} vs {next:?}"
      );
    }
    if 0 < potential_index {
      let (_, prev, _) = &self.ranges[potential_index - 1];
      assert!(
        prev.end <= source.start,
        "New range overlaps at the bottom {source:?} vs {prev:?}"
      );
    }
    self.ranges.insert(potential_index, (id, source, target))
  }

  pub fn resolve(&self, input: T) -> Option<(usize, T)> {
    match self.ranges.binary_search_by_key(&&input, |(_, r, _)| &r.start) {
      Ok(i) => {
        let (id, _, target) = &self.ranges[i];
        println!("Exactly matches {id}, returning target {target:?}");
        Some((*id, target.clone()))
      },
      Err(0) => {
        println!("No matches, too small! {input:?}");
        None
      },
      Err(next) => {
        let (id, source, target) = self.ranges[next - 1].clone();
        if input < source.end {
          let offset = input - source.start.clone();
          let result = target.clone() + offset.clone();
          println!(
            "Within {id} source={source:?}, offset={offset:?}, target={target:?}, mapping to {result:?}"
          );
          Some((id, result))
        } else {
          println!("No matches, {input:?} outside {source:?}");
          None
        }
      },
    }
  }

  pub fn change(&self, input: T) -> (Option<usize>, T) {
    match self.resolve(input.clone()) {
      None => (None, input),
      Some((id, result)) => (Some(id), result),
    }
  }
}

pub fn parse_input(data: &str) -> (Vec<u64>, HashMap<String, RangeMap<u64>>) {
  let (seeds_line, tables) = data.split_once('\n').unwrap();
  let seeds = (seeds_line.strip_prefix("seeds: ").unwrap())
    .split(' ')
    .map(|s| s.parse().unwrap())
    .collect();
  let maps = (tables.split("\n\n"))
    .map(|s| {
      let (name, triples) = s.trim().split_once(" map:\n").unwrap();
      let mut map = RangeMap::<u64>::new();
      for (id, triple) in triples.split('\n').enumerate() {
        let (dest, source, len) = triple
          .split(' ')
          .map(|s| s.trim().parse().unwrap())
          .collect_tuple()
          .unwrap();
        map.add_remap(id, source..source + len, dest)
      }
      (name.to_string(), map)
    })
    .collect();
  (seeds, maps)
}

pub fn day5() {
  let (seeds, maps) = parse_input(&fetch_input(5));
  let seed_to_soil = maps.get("seed-to-soil").unwrap();
  let soil_to_fertilizer = maps.get("soil-to-fertilizer").unwrap();
  let fertilizer_to_water = maps.get("fertilizer-to-water").unwrap();
  let water_to_light = maps.get("water-to-light").unwrap();
  let light_to_temperature = maps.get("light-to-temperature").unwrap();
  let temperature_to_humidity = maps.get("temperature-to-humidity").unwrap();
  let humidity_to_location = maps.get("humidity-to-location").unwrap();
  let min = seeds
    .iter()
    .map(|seed| {
      println!("Processing seed {seed}");
      let (sid, soil) = seed_to_soil.change(*seed);
      let (fid, fertilizer) = soil_to_fertilizer.change(soil);
      let (wid, water) = fertilizer_to_water.change(fertilizer);
      let (lid, light) = water_to_light.change(water);
      let (tid, temp) = light_to_temperature.change(light);
      let (hid, humidity) = temperature_to_humidity.change(temp);
      let (locid, location) = humidity_to_location.change(humidity);
      location
    })
    .min()
    .unwrap();
  println!("The nearest seed is {min}")
}
