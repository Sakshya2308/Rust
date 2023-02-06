#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, Read};
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    //loops

    // for i in arr_2.iter() {
    //     println!("{}", i);
    // }
    let mut i = 0;
    while (i < arr_2.len()) {
        println!("{}", arr_2[i]);
        i += 1;
    }

    
