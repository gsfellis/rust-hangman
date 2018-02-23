extern crate rand;

use std::io;
//use std::cmp::Ordering;
//use rand::Rng;
//use std::process::Command;

struct Phobia {
    name: String,
    hint: String,
    secret_word: String,
}

impl Phobia {
    fn getHint(&self) -> &str { &self.hint }
    fn getWord(&self) -> &str { &self.secret_word }
    
    fn revealLetter(&mut self, _i: &u8) {
        if self.secret_word.is_empty() { return; }
        
    }
}

fn build_phobia (name: String, hint: String) -> Phobia {
        let mut secret_word = "_".repeat(name.len()); 
        Phobia { name, hint, secret_word }
}

fn clear_console() {
    println!{"{}", 27 as char};
}

//fn update_gallow(g: &Vec<String>, x: &u8) -> Vec<String> {

//}

fn show_gallow(g: &Vec<String>) {    
    println!{""};
    println!{" |===="};
    println!{" |   0"};
    println!{" |  /|\\"};
    println!{" |  / \\"};
    println!{"===   "};
    println!{""};
}


fn main() {
    // Variables
    let phobia = build_phobia(String::from("tachophobia"), String::from("speed"));
    
    let mut guesses: u8 = 0;    
    
    let mut gallow: Vec<String> = Vec::new();
    gallow.push(" |==== ".to_string());
    gallow.push(" |     ".to_string());
    gallow.push(" |     ".to_string());
    gallow.push(" |     ".to_string());
    gallow.push("===".to_string()); 

    while guesses != 6 {
        clear_console();

        println!{"=== H A N G M A N ==="};
        
        //update_gallow(&gallow, &guesses);
        show_gallow(&gallow);
        
        println!{"{}", phobia.getHint()};       

        println!{"{}", phobia.getWord()};
        

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: char = match guess.trim().parse() {
            Ok(c) => c,
            Err(_) => continue,
        };

        guesses = guesses + 1;
        println!{"You guessed: {}", guess};
    }
}