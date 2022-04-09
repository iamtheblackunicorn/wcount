/*
WCOUNT by Alexander Abraham,
a.k.a. "Angeldust Duke", a.k.a. "The Black Unicorn".
Licensed under the MIT license.
*/

use std::env;
use std::fs::read_to_string;

// Checks whether a file exists and
/// returns a boolean to that effect.
fn file_is(filename: String) -> bool {
    let mut result: Vec<bool> = Vec::new();
    let contents = read_to_string(filename);
    match contents {
        Ok(_n) => result.push(true),
        Err(_x) => result.push(false)
    }
    return result[0];
}

/// Tries to read a file and return
/// its contents.
fn read_file(filename: String) -> String {
    let mut result: String = String::from("");
    let fname_copy: String = filename.clone();
    if file_is(filename) == true {
        result = read_to_string(fname_copy).unwrap();
    }
    else {}
    return result;
}

// Returns a vector of strings from a character split for a string.
/// Both the string and split character have to be strings.
fn clean_split(subject: String, split_char: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for item in subject.split(&split_char) {
        let new_item: String = item.to_string();
        result.push(new_item);
    }
    return result;
}

fn count_words(src: String) -> usize {
    let mut result: usize = 0;
    let src_clone_one = src.clone();
    let src_clone_two = src_clone_one.clone();
    for _line in clean_split(read_file(src_clone_one), "\n".to_string()){
        let line_word_count = clean_split(_line, " ".to_string()).len();
        result = result + line_word_count;
    }
    return result;
}

/// WCOUNT's tiny-ass CLI.
fn cli() {
    let args: Vec<String> = env::args().collect();
    let arg_len = args.len();
    if arg_len == 2 {
        if file_is(args[1].clone()) {
            println!("{:?} Words", count_words(args[1].clone()));
        }
        else {error_message();}
    }
    else {error_message();}
}

/// A small error message.
fn error_message(){
    println!("Wrong usage!");
}

/// Main entry point for the Rust compiler.
fn main(){
    cli();
}
