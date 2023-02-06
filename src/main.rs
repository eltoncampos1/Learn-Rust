#![allow(unused)]

use std::io;
use std::ops::Add;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;

mod restaurant;
use crate::restaurant::order_food;

fn main() {
   order_food(); 
}
