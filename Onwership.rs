#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, Read};
use std::io::{BufRead, BufReader, ErrorKind, Write};

use std::ops::Add; // allows performaing operations with generic
//Ownership- memory is going to be manage through a system of ownership with rules that act at compile time 
 // RULES
    // 1. Each value has a variable that's called its owner
    // 2. There is only one owner at a time
    // 3. When the owner goes out of scope the value disappears
fn print_str(x: String){
    println!("A string {}", x);
}

fn print_return_str(x: String)->String{
    println!("A string {}", x);
    x
}

fn change_string(name: &mut String){
    name.push_str(" is happy");
    println!("Message : {}", name);
}

    fn main() {
    let mut str1 = String::from("Sakshya");
    // let str2 = str1;
    // let str2 = str1.clone();
    // // print_str(str1);
    // let str3 = print_return_str(str1);
    // println!("str3 = {}", str3);
    change_string(&mut str1);
}
