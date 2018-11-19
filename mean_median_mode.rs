#![allow(unused_variables)]
extern crate rand;
use rand::Rng;
//use rand::{OsRng, Rng};
use std::collections::HashMap;

/*
From the challenge at the bottom of this page:
- https://doc.rust-lang.org/book/2018-edition/ch08-03-hash-maps.html

Given a list of integers, return:
- the mean (the average value), 
- median (when sorted, the value in the middle position),
- and mode (the value that occurs most often)

Pro Tip: use a hashmap and vector
*/

// Random Vector Generator
// TODO: use a cryptographically secure RNG 
fn rvg(lower_bound: i32, 
       upper_bound: i32, 
       length: i32) -> Vec<i32> {
    
    let mut v: Vec<i32> = Vec::new();
    
    for i in (0..length) {
        v.push(rand::thread_rng().gen_range(lower_bound, upper_bound));
    }
    
    v
}

// Function to find the mean of a vector
fn mean(vec: &Vec<i32>) -> i32 {
    
    let mut v = vec.clone();
    
    let len = v.len() as i32;
    let sum: i32 = v.iter().sum();
    let mean = sum/len;
    
    mean
}

// Function to find the median of a vector
fn median(vec: &Vec<i32>) -> i32 {
   
    let mut v = vec.clone();
   
    v.sort();
    let len = v.len();
    let median = v[len/2];
    
    median
}

// Function to find the mode of a vector
fn mode(vec: &Vec<i32>) -> i32 {
    
    let mut v = vec.clone();
    let mut hashmap = HashMap::new();
    
    for num in v {
        let count = hashmap.entry(num).or_insert(0);
        *count += 1;
    }
    
    let mut top = 0;
    let mut mode: i32 = 0;
    for (num, count) in hashmap {
        if count > top {
            top = count;
            mode = num;
        } 
    }
    
    mode
}

// MAIN
fn main() {
    
    // generate a list
    let list = rvg(0,10,10);
    
    // print stats
    println!("list: {:?}", &list);
    println!("mean: {}", mean(&list));
    println!("median: {}", median(&list));
    println!("mode: {}", mode(&list));
    
}
