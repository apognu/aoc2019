use chrono::*;
use regex::{Match, Regex};
use std::collections::HashMap;

use crate::util;

pub fn run() {
  let rgx =
    Regex::new(r"\[1518-\d+-(\d+) \d+:(\d+)\] Guard #(\d+) begins shift")
      .unwrap();
  let date_rgx = Regex::new(r"\[(1518-(\d+)-(\d+) \d+:(\d+))\] .*").unwrap();
  let mut file = util::read_file_by_lines(2018, 4);

  file.sort_by(|line1, line2| {
    let line1_captures = date_rgx.captures(line1).unwrap();
    let line2_captures = date_rgx.captures(line2).unwrap();

    let line1_time = DateTime::parse_from_str(
      &format!("{}:00 +00:00", line1_captures.get(1).unwrap().as_str()),
      "%Y-%m-%d %H:%M:%S %z",
    )
    .unwrap();

    let line2_time = DateTime::parse_from_str(
      &format!("{}:00 +00:00", line2_captures.get(1).unwrap().as_str()),
      "%Y-%m-%d %H:%M:%S %z",
    )
    .unwrap();

    line1_time.cmp(&line2_time)
  });

  let mut timetable: HashMap<u32, Vec<Vec<u32>>> = HashMap::new();
  let mut guard = 0;
  let mut asleep = 0;

  for line in &file {
    if line.contains("begins shift") {
      let captures = rgx.captures(&line).unwrap();

      guard = cast_capture(captures.get(3));
    } else {
      let captures = date_rgx.captures(&line).unwrap();
      let minute = cast_capture(captures.get(4));

      if line.contains("falls asleep") {
        asleep = minute;
      } else if line.contains("wakes up") {
        timetable
          .entry(guard)
          .and_modify(|g| g.push((asleep..minute).collect::<Vec<u32>>()))
          .or_insert_with(|| vec![(asleep..minute).collect::<Vec<u32>>()]);
      }
    }
  }
  let (id, minutes) = timetable
    .clone()
    .into_iter()
    .map(|(id, minutes)| {
      (id, minutes.into_iter().flatten().collect::<Vec<_>>())
    })
    .max_by_key(|(_, minutes)| minutes.len())
    .unwrap();

  let (minute, _) = minutes
    .iter()
    .fold(HashMap::new(), |acc, minute| {
      let mut acc = acc;
      acc.entry(*minute).and_modify(|m| *m += 1).or_insert(1);
      acc
    })
    .into_iter()
    .max_by_key(|(_, minute)| *minute)
    .unwrap();

  println!(
    "Best guard is #{}, most asleep at minute {}: {}.",
    id,
    minute,
    id * minute
  );

  let minutes: Vec<(u32, HashMap<u32, u32>)> = timetable
    .into_iter()
    .map(|(id, minutes)| {
      (id, minutes.into_iter().flatten().collect::<Vec<_>>())
    })
    .map(|(id, minutes)| {
      (
        id,
        minutes.iter().fold(HashMap::<u32, u32>::new(), |acc, minute| {
          let mut acc = acc;
          acc.entry(*minute).and_modify(|m| *m += 1).or_insert(1);
          acc
        }),
      )
    })
    .collect();

  let (mut id, mut minute, mut occurence) = (0, 0, 0);

  for (guard, minutes) in &minutes {
    for (min, count) in minutes {
      if *count > occurence {
        minute = *min;
        id = *guard;
        occurence = *count;
      }
    }
  }

  println!(
    "Best guard is #{}, with best minute {}: {}.",
    id,
    minute,
    id * minute
  );
}

fn cast_capture(capture: Option<Match>) -> u32 {
  capture.unwrap().as_str().parse::<u32>().unwrap()
}
