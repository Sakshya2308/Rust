#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, Read};
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn get_2(list: &[i32]) -> i32{  // function argumens -> what it is returning
   let mut sum = 0;
   for &val in list.iter(){
    sum += &val;
   }
   sum
}
fn main() {
    // let (val_1, val_2) = get_2(3);
    // println!("Nums : {} {}", val_1,val_2);
   let num_list = vec![1,2,3,4,5];
   println!("Sum of list => {}", get_2(&num_list));
}
