#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
 let st1 = String::from("a f j o u r d n l mm b");
 let mut v1: Vec<char> = st1.chars().collect();
 v1.sort();
 v1.dedup();
 for char in v1 {
    println!("{}", char);
 }

 let st2: &str = "Ramdom string";
 let mut str3: String = st2.to_string();
 println!("{}", str3);
 let  byte_arr1 = str3.as_bytes();
 let st4 = &str3[0..6];
 println!("String length: {}", st4.len());
 str3.clear();
 let st4  = String::from("Just SOme");
 let st5  = String::from("worss");
 let st6 = st4 + &st5;
 for char in st6.bytes() {
    println!("Char {}", char)
 }
}
