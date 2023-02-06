#![allow(unused)]

use std::io;
use std::ops::Add;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn print_str(x: String) {
    println!("A string: {}", x)
}

fn print_and_return_str(x: String) -> String {
    println!("A string: {}", x);
    return x;
}

fn change_str(name: &mut String) {
    name.push_str(" is happy!");
    println!("Message: {}", name);
}

fn main() {
   let mut str1 = String::from("Foo");
   change_str(&mut str1);
}
