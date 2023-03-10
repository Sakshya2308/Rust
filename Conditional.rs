#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, Read};
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let age2 = 19;
    match age2 {
        1..=18 => println!("Important Birthday"),
        21 | 50 => println!("Important Birthday"),
        65..=i32::MAX => println!("Important Birthday"),
        _ => println!("Not an important Birthday"),
    };
}

//By using ordering
let my_age = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("can't vote"),
        Ordering::Greater => println!("Can vote"),
        Ordering::Equal => println!("You just gained voting right"),
    };
