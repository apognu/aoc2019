use std::f64::EPSILON;

use crate::util;

type Coords = (isize, isize);
type Angle = f64;

#[derive(Debug)]
struct Angles(Vec<Angle>);

impl Angles {
  fn iter(&self) -> AnglesIter {
    AnglesIter { inner: self, last_angle: -1.0, pos: 0 }
  }
}

struct AnglesIter<'a> {
  inner: &'a Angles,
  last_angle: f64,
  pos: usize,
}

impl<'a> Iterator for AnglesIter<'a> {
  type Item = &'a Angle;

  fn next(&mut self) -> Option<Self::Item> {
    if self.pos >= self.inner.0.len() {
      None
    } else {
      let new_angle = loop {
        self.pos += 1;
        match self.inner.0.get(self.pos - 1) {
          Some(angle) if (angle - self.last_angle).abs() < EPSILON => continue,
          Some(angle) => break angle,
          _ => return None,
        }
      };

      self.last_angle = *new_angle;

      Some(new_angle)
    }
  }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
struct Asteroid {
  x: isize,
  y: isize,
  angle: Angle,
  distance: usize,
}

impl Asteroid {
  fn from(center: &Coords, coords: Coords) -> Self {
    let (center_x, center_y) = center;
    let (x, y) = coords;

    let mut asteroid =
      Asteroid { x: x - center_x, y: -(y - center_y), ..Default::default() };

    asteroid.angle = (asteroid.x as f64).atan2(asteroid.y as f64);
    asteroid.distance = (asteroid.x.abs() + asteroid.y.abs()) as usize;
    asteroid
  }

  fn denormalize(&mut self, center: Coords) {
    let (center_x, center_y) = center;
    self.x += center_x;
    self.y = -self.y + center_y;
  }
}

pub fn run() {
  let file: Vec<Vec<char>> = util::read_file_by_lines(2019, 10)
    .iter()
    .map(|line| line.chars().collect())
    .collect();

  let mut asteroids: Vec<(isize, isize)> = vec![];

  for (y, row) in file.iter().enumerate() {
    for (x, c) in row.iter().enumerate() {
      if c == &'#' {
        asteroids.push((x as isize, y as isize));
      }
    }
  }

  let (station, asteroids_in_sight) = best_monitoring_station(&asteroids);

  println!("Best monitoring station: ({}, {})", station.x, station.y);
  println!("Asteroids in sight: {}", asteroids_in_sight);

  let asteroid =
    search_and_destroy(&asteroids, (station.x, station.y)).unwrap();

  println!(
    "200th asteroid destroyed: ({}, {}) -> {}",
    asteroid.x,
    asteroid.y,
    (asteroid.x * 100) + asteroid.y
  );
}

fn normalize_to_center(center: Coords, asteroids: &[Coords]) -> Vec<Asteroid> {
  let asteroids: Vec<Asteroid> = asteroids
    .iter()
    .map(|asteroid| Asteroid::from(&center, *asteroid))
    .collect();

  asteroids
}

fn all_angles_to_asteroids(asteroids: &[Asteroid]) -> Angles {
  let mut angles: Vec<Angle> =
    asteroids.iter().map(|asteroid| asteroid.angle).collect();

  angles.sort_by(|angle1, angle2| {
    let angle1 = angle1.to_degrees();
    let angle2 = angle2.to_degrees();

    let angle1 = if angle1 < 0.0 { angle1 + 360.0 } else { angle1 };
    let angle2 = if angle2 < 0.0 { angle2 + 360.0 } else { angle2 };

    angle1.partial_cmp(&angle2).unwrap()
  });

  Angles(angles)
}

fn closest_asteroid_at_angle(
  asteroids: &[Asteroid], angle: Angle,
) -> Option<&Asteroid> {
  asteroids
    .iter()
    .filter(|asteroid| (asteroid.angle - angle).abs() < EPSILON)
    .min_by_key(|asteroid| asteroid.distance)
}

fn best_monitoring_station(asteroids: &[Coords]) -> (Asteroid, usize) {
  asteroids
    .iter()
    .map(|(x, y)| {
      let mut asteroid = Asteroid::from(&(*x, *y), (*x, *y));
      let asteroids = normalize_to_center((*x, *y), asteroids);
      let angles = all_angles_to_asteroids(&asteroids);

      let visible_asteroids = angles
        .iter()
        .map(|angle| closest_asteroid_at_angle(&asteroids, *angle))
        .filter(|angle| angle.is_some())
        .count();

      asteroid.denormalize((*x, *y));

      (asteroid, visible_asteroids)
    })
    .max_by_key(|(_, visible_asteroids)| *visible_asteroids)
    .unwrap()
}

fn search_and_destroy(
  asteroids: &[Coords], station: Coords,
) -> Option<Asteroid> {
  let mut asteroids: Vec<Asteroid> = asteroids
    .iter()
    .filter(|asteroid| *asteroid != &station)
    .map(|asteroid| Asteroid::from(&station, *asteroid))
    .collect();

  let angles = all_angles_to_asteroids(&asteroids);

  let mut destroyed = 0;
  let mut target: Option<Asteroid> = None;

  loop {
    if asteroids.is_empty() {
      break;
    }
    angles.iter().for_each(|angle| {
      let asteroid = asteroids
        .clone()
        .into_iter()
        .filter(|asteroid| (*angle - asteroid.angle).abs() < EPSILON)
        .min_by_key(|asteroid| asteroid.distance);

      if let Some(mut asteroid) = asteroid {
        asteroids.remove_item(&asteroid);

        destroyed += 1;
        if destroyed == 200 {
          asteroid.denormalize(station);
          target = Some(asteroid);
        }
      }
    });
  }

  target
}
