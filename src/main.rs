extern crate matrix_lib;
extern crate time;

fn main() {
  use matrix_lib::matrix::{Matrix, random};
  use time::PreciseTime;
  let start = PreciseTime::now();
  let mut m1: Matrix<f64> = random(100, 100);
  let m2: Matrix<f64> = random(100, 100);
  for i in 0..1000 {
    if 0 == i % 100 {
      m1 = m1.clone() * m2.clone();
    }
    m1 = m1.clone() + m2.clone();
    m1 = m1.clone() - m2.clone();
  }
  let end = PreciseTime::now();
  println!("Benchmark executed in {} milliseconds.", start.to(end).num_milliseconds());
}
