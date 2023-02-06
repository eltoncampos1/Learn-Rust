#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random: {}", random_num)

}
