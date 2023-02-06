#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
  let arr_1 = [1,2,3,4,5,6,7,8,9];

  let mut idx = 0;

  for val in arr_1.iter() {
    println!("val: {}", val)
  }   
}
