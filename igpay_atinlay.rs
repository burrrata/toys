/*
Converts strings to pig latin. 

Words that start with a vowel:
- “hay” is added to the end 
- “apple” becomes “apple-hay”

Words that start with a consonant:
- the first letter of is moved to the end of the word
and “ay” is added
- “first” becomes “irst-fay.\” 
*/


// splits words in the sting up into strings in a vector
fn ordsway2ecvay(string: &str) -> Vec<String> {
    
    let mut v: Vec<String> = Vec::new();
    
    for i in string.split_whitespace() {
        v.push(i.to_string())
    }
    
    v
}

// encrypts plaintext to piglatin ciphertext
fn encrypt_hay(vowels: &str,
               plaintext: &str) 
               -> String {
    
    let str = plaintext.to_lowercase();
    let mut chars = str.chars();
    let first_char = chars.next().unwrap();
    
    if vowels.contains(first_char) {
        let output = format!("{}-hay", str);
        output
    } else {
        let output = format!("{}-{}ay", chars.as_str(), first_char);
        output
    }
}

// uses piglatin syntax to encrypt plaintext to ciphertext
fn igpay_atinlay(vowels: &str,
                 plaintext: &str) 
                 -> String {
    
    let vec = ordsway2ecvay(plaintext);
    
    let mut v = Vec::new();
    for i in vec {
        v.push(encrypt_hay(vowels, &i))
    }
    
    let s: String = v.join(" ");
    
    s
}

fn main() {

    let vowels = "aeiou";
    let string = "pig latin";

    println!("{}", igpay_atinlay(vowels, string));
    
}
