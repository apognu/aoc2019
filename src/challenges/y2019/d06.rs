use std::{
  cmp,
  collections::{hash_map::Iter, HashMap},
};

use crate::util;

#[derive(Debug, Default, Clone)]
struct Planet {
  name: String,
  orbit: Option<String>,
}

impl<'a> Planet {
  fn new<S>(name: S, orbit: Option<S>) -> Planet
  where
    S: Into<String>,
  {
    Planet { name: name.into(), orbit: orbit.map(|s| s.into()) }
  }
}

#[derive(Debug, Default)]
struct Arena(HashMap<String, Planet>);

impl Arena {
  fn get<S>(&self, planet: S) -> Option<&Planet>
  where
    S: Into<String>,
  {
    self.0.get(&planet.into())
  }

  fn add(&mut self, planet: Planet) {
    self.0.insert(planet.name.clone(), planet);
  }

  fn iter(&self) -> Iter<'_, String, Planet> {
    self.0.iter()
  }
}

pub fn run() {
  let orbits = util::read_file_by_lines(2019, 6);
  let mut list: HashMap<String, Vec<String>> = HashMap::new();
  let mut arena = Arena::default();

  for planet in orbits {
    let tokens = planet.split(')').collect::<Vec<&str>>();
    let (center, planet) = (tokens[0].to_string(), tokens[1].to_string());

    list.entry(center).or_insert_with(Vec::new).push(planet);
  }

  arena.add(Planet::new("COM", None));

  find_orbiting_planets(&list, &mut arena, "COM".to_string());

  let distance = arena
    .iter()
    .fold(0, |acc, (_, planet)| acc + find_distance(&arena, planet));

  println!("Global orbital distance: {}", distance);

  let mut you = find_path(&arena, arena.get("YOU").unwrap());
  let mut san = find_path(&arena, arena.get("SAN").unwrap());
  let min = cmp::min(you.len(), san.len());
  for _ in 0..min {
    if you[0] != san[0] {
      break;
    }
    you.remove(0);
    san.remove(0);
  }

  println!("Distance between YOU and SAN: {}", you.len() + san.len());
}

fn find_orbiting_planets(
  list: &HashMap<String, Vec<String>>, arena: &mut Arena, center: String,
) {
  if let Some(planets) = list.get(&center) {
    for planet in planets {
      arena.add(Planet::new(planet.clone(), Some(center.clone())));

      find_orbiting_planets(list, arena, planet.to_string());
    }
  }
}

fn find_path(arena: &Arena, planet: &Planet) -> Vec<String> {
  match planet.orbit {
    None => vec![],
    Some(ref orbit) => match arena.get(orbit) {
      None => vec![orbit.clone()],
      Some(planet) => {
        let current = vec![orbit.clone()];
        let mut further = find_path(arena, planet);

        further.extend(current);
        further
      }
    },
  }
}

fn find_distance(arena: &Arena, planet: &Planet) -> i32 {
  match planet.orbit {
    None => 0,
    Some(ref planet) => match arena.get(planet) {
      None => 1,
      Some(planet) => 1 + find_distance(arena, planet),
    },
  }
}
