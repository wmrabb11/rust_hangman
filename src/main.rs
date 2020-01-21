extern crate clap;
use clap::{Arg, App};
use std::fs;
use std::io;
use std::vec::Vec;

fn welcome() {
    println!("Welcome to the Rust CLI Hangman!");
    println!("You have 6 incorrect guesses to make before you lose!");
}

fn print_incorrect_guesses(incorrect_guesses: &[char], remaining: i32) {
    print!("[*] Here are your incorrect guesses: ");
    for (i, c) in incorrect_guesses.iter().enumerate() {
        if i == incorrect_guesses.len() - 1 {
            print!("{}", c);
        } else {
            print!("{}, ", c);
        }
    }
    print!(" ({} left)\n", remaining);
}

fn check_char(to_check: char, solution: &mut String) -> bool {
    if solution.contains(to_check) {
        return true;
    } else {
        return false;
    }
}

fn print_hangman(correct_guesses: &[char], solution: &mut String) {
    print!("[*] Hangman board: ");
    for c in solution.chars() {
        if correct_guesses.len() != 0 {
            if correct_guesses.contains(&c) || c == ' '{
                print!("{}", c);
            } else {
                print!("_");
            }
        } else {
            if c == ' ' {
                print!("{}", c);
            } else {
                print!("_");
            }
        }
    }
    print!("\n");
}

fn check_solved(correct_guesses: &[char], solution: &mut String) -> bool {
    for c in solution.chars() {
        if correct_guesses.contains(&c) || c == ' ' {
            continue;
        } else {
            return false;
        }
    }
    return true;
}

fn main() {
    let mut solved = false;
    let mut correct_guesses = Vec::new();
    let mut incorrect_guesses = Vec::new();
    let mut incorrect_guess_count = 6;
    let matches = App::new("CLI Hangman")
        .version("1.0")
        .author("Will Rabb <wmrabb@ncsu.edu>")
        .about("Command Line Version of the Hangman game")
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .value_name("FILE")
             .help("Sets a file to read the hangman word")
             .takes_value(true)
             .required(true))
        .get_matches();

    let config = matches.value_of("config").unwrap();
    let mut contents = fs::read_to_string(config)
        .expect("[-] Something went wrong with reading the file");
    contents.truncate(contents.len() - 1);

    welcome();

    print_hangman(&correct_guesses.as_slice().to_vec(), &mut contents);

    while !solved {
        println!("[*] Please enter a character: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("[-] Could not read from STDIN");
        let to_check = input.chars().next().unwrap();
        let correct_char = check_char(to_check, &mut contents);
        if correct_char {
            correct_guesses.push(to_check);
        } else {
            incorrect_guesses.push(to_check);
            incorrect_guess_count -= 1;
            if incorrect_guess_count == 0 {
                println!("[-] Oh no! You didn't solve it. You suck :/");
                break;
            }
        }

        print_incorrect_guesses(&incorrect_guesses.as_slice().to_vec(), incorrect_guess_count);

        solved = check_solved(&correct_guesses.as_slice().to_vec(), &mut contents);
        if solved {
            println!("[+] Congratulations! You won!");
        }

        print_hangman(&correct_guesses.as_slice().to_vec(), &mut contents);
    }
}
