extern crate rand;

use std::io;
use std::process;

//use std::io::stdout;
//use std::cmp::Ordering;
//use rand::Rng;
//use std::process::Command;

struct Phobia {
    name: String,
    hint: String,
    secret_word: String,
}

impl Phobia {
    fn get_hint(&self) -> &str { &self.hint }
    fn get_word(&self) -> &str { &self.secret_word }
    
    fn reveal_letter(&mut self, _i: &u8) {
        if self.secret_word.is_empty() { return; }
        
    }
}

fn build_phobia (name: String, hint: String) -> Phobia {
        let secret_word = "_".repeat(name.len()); 
        Phobia { name, hint, secret_word }
}

fn build_gallow () -> Vec<String> {
    let mut gallow: Vec<String> = Vec::new();
    gallow.push(" |==== ".to_string());
    gallow.push(" |     ".to_string());
    gallow.push(" |     ".to_string());
    gallow.push(" |     ".to_string());
    gallow.push("===".to_string());

    gallow
}

/*
fn clear_console() {    
    //let cmd = "cls";
    //if cfg!(unix) {
    //    let cmd = "clear";
    //}
    
    Command::new("cls").output().unwrap_or_else(|e| {
        panic!("error executing process: {}", e)
    });

    //println!{"{}", 27 as char};
}
*/

fn show_word(word: &str) {
    for x in word.chars() {
        print!{"{} ", x};
    }

    println!{""};
}

//fn update_gallow(g: &Vec<String>, x: &u8) -> Vec<String> {

//}

fn show_gallow(g: &Vec<String>) {
    for row in g {
        println!{"{}", row };
    }
}

fn main() {
    // Variables
    let phobia = build_phobia(String::from("tachophobia"), String::from("speed"));    
    
    let max_guesses: u8 = 6;
    let mut hint: bool = false;
    
    let mut guesses: u8 = 0;    
    let mut gallow = build_gallow();

    while guesses < max_guesses {
        //clear_console();

        println!{"=== H A N G M A N ==="};
        println!{"Number of guesses: {}\n", guesses};
        
        //update_gallow(&gallow, &guesses);
        show_gallow(&gallow);
        
        if hint {
            println!{"A fear of {}", phobia.get_hint()};       
        }

        show_word(phobia.get_word());

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: char = match guess.trim().parse() {
            Ok(c) => c,
            Err(_) => continue,
        };

        if guess.is_digit(10) {
            match guess.to_digit(10) {
                Some(d) => {
                    match d {
                        1 => {
                            hint = true;
                            continue
                        },
                        0 => process::exit(0),
                        _ => continue
                    }                    
                },
                None => continue,
            };
        } else {
            
        }

        guesses = guesses + 1;
        println!{"You guessed: {}", guess};
    }
}