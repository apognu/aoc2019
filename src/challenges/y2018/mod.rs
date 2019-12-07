mod d01;
mod d02;
mod d03;
mod d04;

pub fn challenges() -> Vec<(&'static str, &'static dyn Fn())> {
  vec![
    ("Chronal Calibration", &d01::run),
    ("Inventory Management System", &d02::run),
    ("No Matter How You Slice It", &d03::run),
    ("Repose Record", &d04::run),
  ]
}
