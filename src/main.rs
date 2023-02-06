#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
  let age2 = 8;
  match age2 {
    1..=18 => println!("Important Bday"),
    21 | 50 => println!("Important Bday"),
    65..=i32::MAX => println!("Important Bday"),
    _ => println!("NOT an Important Bday"),
  };
}
