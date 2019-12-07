use std::collections::HashSet;

use crate::util;

pub fn run() {
  let modulations: Vec<i32> = util::read_file_by_lines(2018, 1)
    .into_iter()
    .map(|modulation| modulation.parse::<i32>().unwrap())
    .collect();

  let frequency: i32 = modulations.iter().sum();

  println!("Resulting frequency is {}.", frequency);
  println!(
    "First repeating frequency is {}",
    find_repeating_frequency(&modulations)
  );
}

fn find_repeating_frequency(modulations: &[i32]) -> i32 {
  let mut frequency = 0;
  let mut frenquencies: HashSet<i32> = HashSet::new();
  loop {
    for modulation in modulations {
      frequency += modulation;

      if frenquencies.contains(&frequency) {
        return frequency;
      }

      frenquencies.insert(frequency);
    }
  }
}
