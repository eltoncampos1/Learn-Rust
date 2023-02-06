#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age = "25";
    let mut age: u32 = age.trim().parse()
        .expect("Age wasen't assigned a number");
    age = age +1;
    println!("I'm {} and i Want ${}", age, ONE_MIL)
}
