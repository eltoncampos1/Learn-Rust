#![allow(unused)]

use std::io;
use std::ops::Add;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
   struct Customer {
    name: String,
    address: String,
    balance: f32,
   }

   let mut bob = Customer{
    name: String::from("Bob"),
    address: String::from("555 main st"),
    balance: 234.67
   };

   bob.address = String::from("505 main st");
   
}
