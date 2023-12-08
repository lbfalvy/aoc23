use std::fmt::Debug;
use std::iter;
use std::ops::{Add, Range, Sub};

use hashbrown::HashMap;
use itertools::Itertools;

use crate::boxed_iter::box_empty;
use crate::common::fetch_input;

#[derive(Debug, Clone)]
struct RangeTf<T> {
  range: Range<T>,
  target: T,
}
impl<T: Clone + Ord + Add<Output = T> + Sub<Output = T>> RangeTf<T> {
  pub fn target_range(&self) -> Range<T> {
    self.range.start.clone() + self.target.clone()
      ..self.range.end.clone() + self.target.clone()
  }
  pub fn contains(&self, t: &T) -> bool { self.range.contains(t) }
  pub fn translate(&self, t: &T) -> Option<T> {
    (self.contains(t))
      .then(|| t.clone() - self.range.start.clone() + self.target.clone())
  }
  pub fn overlaps(&self, target: &Range<T>) -> bool {
    self.range.start < target.start || target.end < self.range.end
  }
}

pub struct RangeMap<T> {
  ranges: Vec<RangeTf<T>>,
}
impl<T: Debug + Clone + Ord + Add<Output = T> + Sub<Output = T>> RangeMap<T> {
  pub fn new() -> Self { Self { ranges: Vec::new() } }
  pub fn add_remap(&mut self, range: Range<T>, target: T) {
    if range.is_empty() {
      return;
    }
    let potential_index = (self.search_by_start(&range.start))
      .expect_err("Two ranges start at the same index");
    if potential_index < self.ranges.len() {
      let next = &self.ranges[potential_index];
      assert!(
        range.end <= next.range.start,
        "New range overlaps at the top {range:?} vs {next:?}"
      );
    }
    if 0 < potential_index {
      let prev = &self.ranges[potential_index - 1];
      assert!(
        prev.range.end <= range.start,
        "New range overlaps at the bottom {range:?} vs {prev:?}"
      );
    }
    self.ranges.insert(potential_index, RangeTf { range, target })
  }

  pub fn search_by_start(&self, start: &T) -> Result<usize, usize> {
    self.ranges.binary_search_by_key(&start, |tf| &tf.range.start)
  }

  pub fn resolve_range(
    &self,
    input: Range<T>,
  ) -> impl Iterator<Item = Range<T>> + '_ {
    if input.start == input.end {
      return box_empty();
    }
    Box::new(
      iter::once(None)
        .chain(
          (self.ranges.iter())
            .filter({ let i = input.clone(); move |tf| tf.overlaps(&i)})
            .cloned()
            .map(Some),
        )
        .chain(iter::once(None))
        .tuple_windows()
        .filter_map({ let i = input.clone(); move |pair| match pair {
          (None, Some(tf)) if i.start < tf.range.start => Some(
            i.start.clone()..tf.range.start.clone(),
          ),
          (Some(tf), None) if tf.range.end < i.end => Some(
            tf.range.end.clone()..i.end.clone()
          ),
          (Some(tf1), Some(tf2)) if tf1.range.end < tf2.range.start => Some(
            tf1.range.end.clone()..tf2.range.start.clone()
          ),
          _ => None
        }})
        // Gaps between transformed rnages
        .chain((self.ranges.iter())
      .filter({let i = input.clone(); move |tf| tf.overlaps(&i) })
      .map({ let i = input; move |tf| {
        let start = tf.translate(&i.start).unwrap_or(tf.target_range().start);
        let end = tf.translate(&i.end).unwrap_or(tf.target_range().end);
        start..end
      }})),
    )
  }
}

pub fn optimize_range_set(mut ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
  ranges.sort_unstable_by_key(|r| r.start);
  let mut old_ranges = ranges.into_iter();
  let mut new_ranges = Vec::new();
  let mut cur = old_ranges.next().unwrap();
  loop {
    match old_ranges.next() {
      None => {
        new_ranges.push(cur);
        return new_ranges;
      },
      Some(new) if new.start <= cur.end => cur.end = new.end,
      Some(new) => {
        new_ranges.push(cur);
        cur = new;
      },
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
      for triple in triples.split('\n') {
        let (dest, source, len) = triple
          .split(' ')
          .map(|s| s.trim().parse().unwrap())
          .collect_tuple()
          .unwrap();
        map.add_remap(source..source + len, dest)
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
  // part 1 can be reconstructed by replacing this with a mapping to 1 element
  // ranges
  let seed_ranges = (seeds.iter())
    .cloned()
    .batching(|it| {
      it.next()
        .map(|l| l..it.next().expect("odd number of seed range components"))
    })
    .collect_vec();
  println!("Collected {} seed ranges", seed_ranges.len());
  let soils = optimize_range_set(
    (seed_ranges.into_iter())
      .flat_map(|seeds| seed_to_soil.resolve_range(seeds))
      .collect(),
  );
  println!("Collected {} soil ranges", soils.len());
  let ferts = optimize_range_set(
    (soils.into_iter())
      .flat_map(|soils| soil_to_fertilizer.resolve_range(soils))
      .collect(),
  );
  println!("Collected {} fertilizer ranges", ferts.len());
  let waters = optimize_range_set(
    (ferts.into_iter())
      .flat_map(|ferts| fertilizer_to_water.resolve_range(ferts))
      .collect(),
  );
  println!("Collected {} water ranges", waters.len());
  let lights = optimize_range_set(
    (waters.into_iter())
      .flat_map(|waters| water_to_light.resolve_range(waters))
      .collect(),
  );
  println!("Collected {} light ranges", lights.len());
  let temps = optimize_range_set(
    (lights.into_iter())
      .flat_map(|lights| light_to_temperature.resolve_range(lights))
      .collect(),
  );
  println!("Collected {} temperature ranges", temps.len());
  let hums = optimize_range_set(
    (temps.into_iter())
      .flat_map(|temps| temperature_to_humidity.resolve_range(temps))
      .collect(),
  );
  println!("Collected {} humidity ranges", hums.len());
  let locs = optimize_range_set(
    (hums.into_iter())
      .flat_map(|hums| humidity_to_location.resolve_range(hums))
      .collect(),
  );
  let min = locs.into_iter().map(|locs| locs.start).min().unwrap();
  println!("The nearest seed is at {min}")
}
