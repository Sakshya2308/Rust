#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, Read};
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let st3 = String::from("x r t b h k k a m c");
    let mut v1: Vec<char> = st3.chars().collect(); // collect characters from the string and store them in vector
    v1.sort();
    v1.dedup(); //remove the duplicates
    for char in v1 {
        println!("{}", char);
    }
    let st4: &str = "Random string";
    let mut st5: String = st4.to_string();
    println!("{}", st5);
    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6];
    println!("String length : {} and the string {}", st6.len(), st6);
    st5.clear();
    let st6 = String::from("Just some");
    let st7 = String::from("Words");
    let st8 = st6 + &st7; // it means that the st6 is deleted and stored in st8 whereas string 7 still exists
}
