#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
   let age = 8;
   if (age >= 1) && (age <= 18) {
    println!("Important Birthday");
   } else if (age == 32) || (age == 50) {
    println!("Important Birthday");

   } else if (age >= 65) {
    println!("Important Birthday");
   } else {
    println!("Not Important Birthday");

   }

}
