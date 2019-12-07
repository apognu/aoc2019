mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;

pub fn challenges() -> Vec<(&'static str, &'static dyn Fn())> {
  vec![
    ("The Tyranny of the Rocket Equation", &d01::run),
    ("1202 Program Alarm", &d02::run),
    ("Crossed Wires", &d03::run),
    ("Secure Container", &d04::run),
    ("Sunny with a Chance of Asteroids", &d05::run),
    ("Universal Orbit Map", &d06::run),
    ("Amplification Circuit", &d07::run),
    ("Space Image Format", &d08::run),
    ("Sensor Boost", &d09::run),
    ("Monitoring Station", &d10::run),
    ("Space Police", &d11::run),
    ("The N-Body Problem", &d12::run),
    ("Care Package", &d13::run),
    ("Space Stoichiometry", &d14::run),
    ("Oxygen System", &d15::run),
  ]
}
