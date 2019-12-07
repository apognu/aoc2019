use std::{
  fs::{self, File},
  io::{prelude::*, BufReader},
  path::Path,
};

fn get_input(year: u16, day: u8) -> String {
  let path = format!("inputs/{}/d{:0>2}.txt", year, day);
  if Path::new(&path).exists() {
    return path;
  }

  panic!("input file does not exist");
}

pub fn read_file_by_lines(year: u16, day: u8) -> Vec<String> {
  let file = File::open(get_input(year, day)).expect("no such file");
  let buf = BufReader::new(file);

  buf
    .lines()
    .map(|line| line.unwrap())
    .filter(|line| !line.is_empty())
    .collect()
}

pub fn read_split_file(year: u16, day: u8, sep: &'static str) -> Vec<String> {
  fs::read_to_string(get_input(year, day))
    .expect("no such file")
    .trim()
    .split(sep)
    .map(|x| x.to_string())
    .collect()
}

pub fn read_chared_file(year: u16, day: u8) -> Vec<String> {
  fs::read_to_string(get_input(year, day))
    .expect("no such file")
    .trim()
    .chars()
    .map(|x| x.to_string())
    .collect()
}
