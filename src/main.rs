#![allow(unused)]

use std::io;
use std::ops::Add;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;


fn main() {
   let mut arr_it = [1,2,3,4,5];
   for val in arr_it.iter() {
    println!("{}", val);
   }

   let mut iter1 = arr_it.iter();
   println!("1st: {:?}", iter1.next());
}
