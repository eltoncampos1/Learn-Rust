#![allow(unused)]

use std::io;
use std::ops::Add;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let mut heroes = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce  Wayne");
    heroes.insert("Flash", "Barry Allen");

    for (k ,v) in heroes.iter() {
        println!("{} = {}", k, v);
    }

    if heroes.contains_key(&"Batman") {
        let the_batman = heroes.get(&"Batman");
        match the_batman {
            Some(x) => println!("Batman is a hero"),
            None => println!("Batman is not a superhero"),
        }
    }
}
