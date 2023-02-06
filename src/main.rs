#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
  let my_tp: (u8, String, f64) = (47, "foo".to_string(), 50_000.00); 

  println!("Name: {}", my_tp.1);
  let(v1, v2, v3) = my_tp;
  println!("Age: {}", v1);

}
