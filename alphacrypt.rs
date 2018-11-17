#![allow(unused_variables)]
use std::collections::HashMap;

// Function to create a HashMap that takes in a string 
// of characters and stores them as keys with the index
// of each string as the value.
fn alpha_hashmap(alphabet_string: &str,
                 mut hashmap: HashMap<char, i32>) 
                 -> HashMap<char, i32> {
    
    // aplphabet_string:
    // a string with characters you are using in the
    // alphabet of your message

    // hashmap:
    // a HashMap initialized with 
    // - a space ' ' as the char
    // - 0 as the i32
    
    // returns the hashmap of type HashMap<char, i32>
    
    let mut count = 1;
    for c in alphabet_string.chars() {
        hashmap.insert(c, count);
        count += 1;
    }

    hashmap
}

// Function to encrypt a plaintext message to  a 
// "ciphertext" based on the values in a HashMap.
fn alpha_encrypt(hashmap: HashMap<char, i32>,
                 plaintext: &str) 
                 -> Vec<i32> {
    
    // hashmap:
    // a HashMap of an alphabet where:
    // - each key is letter in the alphabet
    // - the value of each key is that letter's index 
    //   in the alphabet
    // a = 1, b = 2, c = 3, etc...
    
    // plaintext:
    // a string you want to convert to numerical values
    // based on each letter's index in the alphabet that 
    // hashmap is based on

    // returns a Vec<i32> vector

    let lowercase = plaintext.to_lowercase();
    
    let mut ciphertext: Vec<i32> = Vec::new();
    
    for c in lowercase.chars() {
        let num = hashmap.get(&c).unwrap();
        ciphertext.push(*num);
    }   
    
    ciphertext
}
    
fn main() {

    // choose your alphabet
    let alphabet = "abcdefghijklmnopqrstuvwxyz";

    // create hashmap of alphabet    
    let mut hashmap = HashMap::new();
    hashmap.insert(' ', 0);
    let ah = alpha_hashmap(alphabet, hashmap);

    // encrypt plaintext message and print ciphertext
    let plaintext = "I love you";
    let secret_message = alpha_encrypt(ah, plaintext);
    println!("secret message: {:?}", secret_message);

}
