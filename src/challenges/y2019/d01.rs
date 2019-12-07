use crate::util;

pub fn run() {
  let file = util::read_file_by_lines(2019, 1);
  let masses = file
    .iter()
    .filter(|line| !line.is_empty())
    .map(|line| line.parse::<i32>().expect("invalid input"));

  let fuel_requirement: i32 =
    masses.clone().map(|mass| get_fuel_requirement(mass, false)).sum();

  println!("Fuel required to launch: {}", fuel_requirement);

  let fuel_requirement: i32 =
    masses.map(|mass| get_fuel_requirement(mass, true)).sum();

  println!("Corrected fuel required to launch: {}", fuel_requirement)
}

fn get_fuel_requirement(
  mass: i32, consider_fuel_fuel_requirement: bool,
) -> i32 {
  let needed_fuel = (mass / 3) - 2;

  if needed_fuel > 0 {
    if consider_fuel_fuel_requirement {
      needed_fuel
        + get_fuel_requirement(needed_fuel, consider_fuel_fuel_requirement)
    } else {
      needed_fuel
    }
  } else {
    0
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn part1() {
    assert_eq!(super::get_fuel_requirement(12, false), 2);
    assert_eq!(super::get_fuel_requirement(14, false), 2);
    assert_eq!(super::get_fuel_requirement(1969, false), 654);
    assert_eq!(super::get_fuel_requirement(100756, false), 33583);
  }
  #[test]
  fn part2() {
    assert_eq!(super::get_fuel_requirement(14, true), 2);
    assert_eq!(super::get_fuel_requirement(1969, true), 966);
    assert_eq!(super::get_fuel_requirement(100756, true), 50346);
  }
}
