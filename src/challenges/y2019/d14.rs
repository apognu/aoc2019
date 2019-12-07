use std::{cmp::Ordering, collections::HashMap};

use crate::util;

type Requirements = HashMap<String, (u128, Vec<(u128, String)>)>;

pub fn run() {
  let file = util::read_file_by_lines(2019, 14);
  let raw: Vec<Vec<Vec<Vec<&str>>>> = file
    .iter()
    .map(|line| {
      line
        .split(" => ")
        .map(|t| t.split(", ").map(|t| t.split(' ').collect()).collect())
        .collect()
    })
    .collect();

  let mut requirements: Requirements = HashMap::new();

  for spec in &raw {
    let (rq, rm) = (
      &spec[spec.len() - 1][0][0].parse::<u128>().unwrap(),
      &spec[spec.len() - 1][0][1].to_owned(),
    );

    let components: Vec<(u128, String)> = spec[0..spec.len() - 1][0]
      .iter()
      .map(|req| (req[0].parse::<u128>().unwrap(), req[1].to_owned()))
      .collect();

    requirements.insert(rm.to_owned(), (*rq, components));
  }

  println!(
    "We need {} ORES to produce 1 FUEL.",
    get_ores_for(1, &requirements)
  );

  println!(
    "Fuel produced by one trillion ores: {}",
    search_ores_for(1_000_000_000_000, &requirements)
  );
}

fn get_ores_for(fuel: u128, requirements: &Requirements) -> u128 {
  let mut excesses: HashMap<String, u128> = HashMap::new();
  compute_requirements((fuel, "FUEL".to_string()), &requirements, &mut excesses)
    .iter()
    .map(|(qty, _)| qty)
    .sum()
}

fn search_ores_for(fuel: u128, requirements: &Requirements) -> u64 {
  let mut min: u64 = 0;
  let mut max: u64 = std::u64::MAX;

  loop {
    let index = min + ((max - min) / 2);
    let ores = get_ores_for(index as u128, &requirements);

    match ores.cmp(&fuel) {
      Ordering::Less => min = index,
      Ordering::Greater => max = index,
      Ordering::Equal => (),
    }
    if min == (max - 1) {
      return min;
    }
  }
}

fn compute_requirements(
  wanted: (u128, String), requirements: &Requirements,
  excesses: &mut HashMap<String, u128>,
) -> Vec<(u128, String)> {
  let (wanted_qty, wanted) = wanted;
  let (produced_wanted, submaterials) =
    requirements.get(&wanted).expect("unknown result chemical");
  let multiplier = (wanted_qty as f64 / *produced_wanted as f64).ceil() as u128;
  let excess = if (produced_wanted * multiplier) > wanted_qty {
    (produced_wanted * multiplier) - wanted_qty
  } else {
    0
  };

  let subs = submaterials
    .iter()
    .flat_map(|(produced_req, req)| {
      let required = *produced_req * multiplier;

      if *req == "ORE" {
        vec![(required, req.to_string())]
      } else {
        let left = {
          let excess = excesses.get(req).unwrap_or(&0);
          if *excess > required {
            0
          } else {
            required - excess
          }
        };

        excesses.entry(req.clone()).and_modify(|q| *q -= required - left);

        compute_requirements((left, req.clone()), requirements, excesses)
      }
    })
    .collect();

  excesses.entry(wanted).and_modify(|q| *q += excess).or_insert(excess);

  subs
}
