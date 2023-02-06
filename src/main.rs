#![allow(unused)]

use std::io;
use std::ops::Add;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;


fn main() {
 let mut samp1 = 5;
 let print_var = || println!("samp1  = {}", samp1);
 print_var();
 samp1 = 10;
 let mut change_var = || samp1 +=1;
 change_var();
 println!("samp1 = {} ", samp1);
 samp1 = 10;

 println!("samp1 = {} ", samp1);
}
