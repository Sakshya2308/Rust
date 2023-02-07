#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::io::{BufRead, BufReader, ErrorKind, Write};

use std::ops::Add; 
    fn main() {
       let mut heroes = HashMap::new();
       heroes.insert("Superman", "clark kent");
       heroes.insert("Batman", "Bruce wayne");
       heroes.insert("Flash", "Barry allen");
      for(k,v) in heroes.iter(){
        println!("{} = {}", k, v);
      }
      println!("Length : {}", heroes.len());
      if heroes.contains_key(&"Batman"){
        let the_batman = heroes.get(&"Batman");
        match the_batman{
            Some(x) => print!("Batman is a hero"),
        None => print!("Batman is not a hero"),
        }
      }
}
