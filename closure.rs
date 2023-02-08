#![allow(unused)]

use rand::{Rng, Error};
use core::panic;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::io::{BufRead, BufReader, ErrorKind, Write};

use std::ops::Add;
// use std::str::pattern::CharPredicateSearcher;


//how to open, read and write from file AND error handling
fn main() {
   // closure- function without a name, used to pass one function into another function
   //let var_name = |parameters| -> return_type {BODY}
 
//  // basic type of closure
//    let can_vote = |age: i32| {
//     age >= 18
//  };
//  println!("Can vote : {}", can_vote(8));

// // another example

// let mut samp1 = 5;
// let print_var = || println!("samp1 = {}", samp1);
// print_var();
// samp1 = 10;
// let mut change_var = || samp1 += 1;
// change_var();
// println!("samp1 : {}", samp1);
// samp1 = 10;
// println!("samp1 : {}", samp1);

//generic
fn use_func<T>(a: i32, b: i32, func: T) -> i32
where T: Fn(i32, i32) -> i32 {
    func(a,b)
}
let sum = |a, b| a+b;
let prod = |a, b| a*b;
println!("5 + 4 = {}", use_func(5,4,sum));
println!("5 * 4 = {}", use_func(5,4,prod));
}
