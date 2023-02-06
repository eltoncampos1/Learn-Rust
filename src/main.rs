#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
  let mut my_age = 25;
  let can_vote = if my_age >= 18{
    true
  } else {
    false
  };

  println!("Can vote: {}", can_vote)
}
