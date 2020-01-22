extern crate clap;
use clap::{Arg, App};
use std::fs;
use std::io;
use std::vec::Vec;

mod draw;

/**
 * Displays the welcome message to screen
 */
fn welcome() {
    println!("Welcome to the Rust CLI Hangman!");
    println!("You have 6 incorrect guesses to make before you lose!");
}

/**
 * Prints a comma-separated list of the incorrect guesses and the number of wrong
 * guesses that remain
 * @param incorrect_guesses - the char vector that contains the incorrect guesses
 * @param remaining - the number of wrong guesses remaining
 */
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

/**
 * Checks whether or not a given character is in the solution
 * @param to_check - the character to check if it's in the solution
 * @param solution - the solution string to check against
 * @return bool - true if the character is in the solution, false otherwise
 */
fn check_char(to_check: char, solution: &mut String) -> bool {
    if solution.contains(to_check) {
        return true;
    } else {
        return false;
    }
}

/**
 * Prints the hangman board
 * Prints "_" if the character hasn't been guessed yet
 * Prints the character if it's been correctly identified
 * @param correct_guesses - the char vector of correctly guessed characters
 * @param solution - the string containing the solution
 */
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

/**
 * Checks to see if the puzzle has been completely solved
 * @param correct_guesses - the char vector of correctly guessed characters
 * @param solution - the string containing the solution
 * @return bool - true if the puzzle is completely solved, false otherwise
 */
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

/**
 * The main loop of the program
 */
fn main() {
    /* Keeps track of if the puzzle is completely solved */
    let mut solved = false;

    /* Char vectors containing the characters correctly/incorrectly guessed */
    let mut correct_guesses = Vec::new();
    let mut incorrect_guesses = Vec::new();

    /* Keeps track of how many incorrect guesses the player has left before they lose */
    let mut incorrect_guess_count = 6;

    /* Setup command line arguments and app details */
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

    /* Read the config file for the solution word/phrase */
    let config = matches.value_of("config").unwrap();
    let mut contents = fs::read_to_string(config)
        .expect("[-] Something went wrong with reading the file");
    contents.truncate(contents.len() - 1); // Remove the newline from the string

    welcome();

    print_hangman(&correct_guesses.as_slice().to_vec(), &mut contents);

    /* Input loop until the player wins or loses */
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

            /* Handles when the player runs out of guesses */
            if incorrect_guess_count == 0 {
                println!("[-] Oh no! You didn't solve it. Try Again!");
                print_incorrect_guesses(&incorrect_guesses.as_slice().to_vec(), incorrect_guess_count);
                draw::draw_hangman(incorrect_guess_count);
                break;
            }
        }

        /* Print game data */
        print_incorrect_guesses(&incorrect_guesses.as_slice().to_vec(), incorrect_guess_count);
        draw::draw_hangman(incorrect_guess_count);

        solved = check_solved(&correct_guesses.as_slice().to_vec(), &mut contents);
        if solved {
            println!("[+] Congratulations! You won!");
        }

        print_hangman(&correct_guesses.as_slice().to_vec(), &mut contents);
    }
}
