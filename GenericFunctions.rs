#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, Read};
use std::io::{BufRead, BufReader, ErrorKind, Write};

use std::ops::Add; // allows performaing operations with generic

fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T{
    return x+y;
}
fn main() {
    println!("5 + 4 = {}", get_sum_gen(5,4));

    println!("5.2 + 4.6 = {}", get_sum_gen(5.2,4.6));
}
