#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    enum Days {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }

    impl Days {
        fn is_weekend(&self) -> bool {
            match self {
                Days::Saturday | Days::Sunday => true,
                _ => false
            }
        }
    }

    let today:Days = Days::Monday;
    match today {
        Days::Monday => println!("Everyone hates Monday"),
        Days::Tuesday => println!("Donut day"),
        Days::Wednesday => println!("Hump day"),
        Days::Thursday => println!("Pay day"),
        Days::Friday => println!("Almost weedend"),
        Days::Saturday => println!("Weedend"),
        Days::Sunday => println!("Weedend"),
    }

    println!("Is today the weekend {}", today.is_weekend());
}
