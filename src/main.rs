#![feature(pattern, vec_remove_item)]

mod challenges;
mod util;

use std::{collections::HashMap, env, process};

use crate::challenges::*;

fn main() {
  let mut challenges =
    HashMap::<&str, Vec<(&'static str, &'static dyn Fn())>>::new();

  challenges.insert("2018", y2018::challenges());
  challenges.insert("2019", y2019::challenges());

  let args: Vec<String> = env::args().collect();

  if let (Some(year), Some(day)) = (args.get(1), args.get(2)) {
    if let Some(challenges) = challenges.get(year.as_str()) {
      if let Ok(day) = day.parse::<usize>() {
        run(&challenges, day);
      } else if day == "all" {
        run_all(&challenges);
      } else {
        fatal("Usage: aoc2019 <YEAR> <DAY|all>")
      }
    } else {
      fatal("the provided year was not found")
    }
  } else {
    fatal("Usage: aoc2019 <YEAR> <DAY|all>")
  }
}

fn run(challenges: &[(&str, &dyn Fn())], day: usize) {
  if day < 1 || day > challenges.len() {
    fatal("no challenge for this day (yet)");
  }

  let (name, challenge) = challenges[day - 1];

  println!("# DAY {} - {}", day, name);

  challenge();
}

fn run_all(challenges: &[(&str, &dyn Fn())]) {
  for day in 1..=challenges.len() {
    run(challenges, day);
    println!();
  }

  println!("Merry Christmas!");
}

fn fatal(message: &str) {
  eprintln!("ERROR: {}", message);
  process::exit(1);
}
