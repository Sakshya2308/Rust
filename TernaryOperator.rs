#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, Read};
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let mut my_age = 47;
    let can_vote = if my_age >= 18 { true } else { false };
    println!("Can vote: {}", can_vote);
}
