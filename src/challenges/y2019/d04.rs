use crate::util;

pub fn run() {
  let file = util::read_split_file(2019, 4, "-");
  let range: Vec<u64> =
    file.iter().map(|x| x.parse::<u64>().expect("invalid input")).collect();

  let mut matches = 0;

  for password in range[0]..=range[1] {
    if is_password_ok(password) {
      matches += 1;
    }
  }

  println!("Matches: {}", matches);
  let mut matches = 0;

  for password in range[0]..=range[1] {
    if is_password_really_ok(password) {
      matches += 1;
    }
  }

  println!("Fixed matches: {}", matches);
}

fn is_password_ok(password: u64) -> bool {
  let digits = parse_digits(password);
  let (mut twins, mut previous) = (false, 0);

  for digit in digits {
    if digit < previous {
      return false;
    }
    if digit == previous {
      twins = true;
    }
    previous = digit;
  }
  twins
}

fn is_password_really_ok(password: u64) -> bool {
  let digits = parse_digits(password);

  let (mut repetitions, mut twins, mut previous) = (1, false, 0);

  for digit in digits {
    if digit < previous {
      return false;
    }

    if digit == previous {
      repetitions += 1;
    } else {
      if repetitions == 2 {
        twins = true;
      }

      repetitions = 1;
    }

    previous = digit;
  }

  if repetitions == 2 {
    twins = true
  }

  twins
}

fn parse_digits(x: u64) -> Vec<u64> {
  let digit = x % 10;
  let remainder = x / 10;

  if remainder > 9 {
    let mut left = parse_digits(remainder);

    left.extend(vec![digit]);
    left
  } else {
    vec![remainder, digit]
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn part1() {
    assert_eq!(super::is_password_ok(111111), true);
    assert_eq!(super::is_password_ok(223450), false);
    assert_eq!(super::is_password_ok(123789), false);
  }

  #[test]
  fn part2() {
    assert_eq!(super::is_password_really_ok(112233), true);
    assert_eq!(super::is_password_really_ok(123444), false);
    assert_eq!(super::is_password_really_ok(111122), true);
  }
}
