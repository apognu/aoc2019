use itertools::Itertools;

use crate::util::{self, intcode::Circuit};

pub fn run() {
  let file = util::read_split_file(2019, 7, ",");
  let stack: Vec<i128> = file
    .iter()
    .map(|bit| bit.parse::<i128>().expect("invalid input"))
    .collect();

  let mut outputs: Vec<i128> = vec![];
  for settings in (0..5).map(|_| (0..5)).multi_cartesian_product() {
    if !settings.contains(&0)
      || !settings.contains(&1)
      || !settings.contains(&2)
      || !settings.contains(&3)
      || !settings.contains(&4)
    {
      continue;
    }

    outputs.push(amplify(stack.clone(), settings));
  }

  println!("Output signal: {}", outputs.iter().max().unwrap());

  let mut outputs: Vec<i128> = vec![];
  for settings in (5..10).map(|_| (5..10)).multi_cartesian_product() {
    if !settings.contains(&5)
      || !settings.contains(&6)
      || !settings.contains(&7)
      || !settings.contains(&8)
      || !settings.contains(&9)
    {
      continue;
    }
    outputs.push(amplify_with_feedback_loop(stack.clone(), settings));
  }

  println!(
    "Output signal with feedback loop: {}",
    outputs.iter().max().unwrap()
  );
}

fn amplify(stack: Vec<i128>, settings: Vec<i128>) -> i128 {
  let mut circuit = Circuit::with_copies(
    5,
    stack,
    |_, index, output| vec![settings[index], output],
    false,
  );
  circuit.execute()
}

fn amplify_with_feedback_loop(stack: Vec<i128>, settings: Vec<i128>) -> i128 {
  let mut circuit = Circuit::with_copies(
    5,
    stack,
    |cycle, index, output| {
      if cycle == 0 {
        vec![settings[index], output]
      } else {
        vec![output]
      }
    },
    true,
  );

  circuit.execute()
}

#[cfg(test)]
mod tests {
  #[test]
  fn part1() {
    let data = vec![
      (
        vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0],
        vec![4, 3, 2, 1, 0],
        43210,
      ),
      (
        vec![
          3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1,
          24, 23, 23, 4, 23, 99, 0, 0,
        ],
        vec![0, 1, 2, 3, 4],
        54321,
      ),
      (
        vec![
          3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33,
          1002, 33, 7, 33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ],
        vec![1, 0, 4, 3, 2],
        65210,
      ),
    ];

    for (stack, settings, result) in data {
      let output = super::amplify(stack, settings);

      assert_eq!(output, result);
    }
  }

  #[test]
  fn part2() {
    let data = vec![
      (
        vec![
          3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4,
          27, 1001, 28, -1, 28, 1005, 28, 6, 99, 0, 0, 5,
        ],
        vec![9, 8, 7, 6, 5],
        139629729,
      ),
      (
        vec![
          3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005,
          55, 26, 1001, 54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0,
          55, 1001, 55, 1, 55, 2, 53, 55, 53, 4, 53, 1001, 56, -1, 56, 1005,
          56, 6, 99, 0, 0, 0, 0, 10,
        ],
        vec![9, 7, 8, 5, 6],
        18216,
      ),
    ];

    for (stack, settings, result) in data {
      let output = super::amplify_with_feedback_loop(stack, settings);

      assert_eq!(output, result);
    }
  }
}
