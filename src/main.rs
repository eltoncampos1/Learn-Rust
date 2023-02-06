#![allow(unused)]

use std::io;
use std::ops::Add;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;


fn main() {
     fn use_func<T>(a: i32, b: i32, func: T) -> i32 
     where T: Fn(i32, i32) -> i32 {
        func(a,b)
     }
     let sum = |a: i32, b: i32| a+b;
     let prod = |a: i32, b: i32| a*b;
     println!("5 + 4 = {}", use_func(5, 4, sum));
     println!("5 * 4 = {}", use_func(5, 4, prod));
}
