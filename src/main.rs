#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
  let my_age  = 18;
  let voting_age = 18;

  match my_age.cmp(&voting_age) {
    Ordering::Less => println!("Cant Vote"),
    Ordering::Greater => println!("Cant Vote"),
    Ordering::Equal => println!("Can Vote"),
  };
}
