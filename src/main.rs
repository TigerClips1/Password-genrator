#![deny(clippy::all)]

use std::io::stdin;

use rand::prelude::*;

static DIGITS: &'static [char] = & ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];


static LOCASE_CHARACTERS: & 'static [char] = & 
[
    'a',  'b', 'c', 'd', 'e', 'f', 'g', 'h', 
    'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 
    'q', 'r', 's', 't', 'u', 
    'v', 'w', 'x', 'y', 'z'
]; 
  
static UPCASE_CHARACTERS: & 'static [char]  = & 
[   'A',  'B', 'C', 'D', 'E', 'F', 'G', 'H', 
    'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 
    'Q', 'R', 'S', 'T', 'U', 
    'V', 'W', 'X', 'Y', 'Z'
]; 
  
static SYMBOLS: & 'static [char] = & [
   '@', '#', '$', '%', '=', ':', '?', '.', '/', '|', '~', '>',  
        '*', '(', ')', '<', 
        '^', '&', '-', '_', 
        '+', '`', ',', 
        '{', '}', ';'
]; 

fn main() {
    println!("This is a password generator");
    println!("Enter the length of the password you want to generate");
    let mut password_length = String::new();
    stdin().read_line(&mut password_length).unwrap();
  
    let password_length = password_length.trim().parse::<u32>().expect("Invalid length");
  
    generate_password(password_length)
  }
  
  fn generate_password(password_length: u32) {
      let combined: &[char]  = & [DIGITS , LOCASE_CHARACTERS, UPCASE_CHARACTERS, SYMBOLS].concat();
      let mut characters: Vec<String> = Vec::new();    
      for _i in 1..=password_length {
          characters.push(combined.choose(&mut rand::thread_rng()).unwrap().to_string())
      }
  
      let password: String = characters.join("");
      println!("The password is: {password}");
  }

