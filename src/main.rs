#![allow(unused)]

use rand::prelude::*;
use std::io::{Write, stdin, stdout};

const WORDS: &[&'static str] = &[
    "icicle",
    "pizza",
    "slough",
];

const MEN: &[[[char; 3]; 4]] = &[
    [[' ', ' ', ' '],
     [' ', ' ', ' '],
     [' ', ' ', ' '],
     [' ', ' ', ' ']],
    [[' ', 'O', ' '],
     [' ', ' ', ' '],
     [' ', ' ', ' '],
     [' ', ' ', ' ']],
    [[' ', 'O', ' '],
     [' ', '|', ' '],
     [' ', '|', ' '],
     [' ', ' ', ' ']],
    [[' ', 'O', ' '],
     ['\\', '|', ' '],
     [' ', '|', ' '],
     [' ', ' ', ' ']],
    [[' ', 'O', ' '],
     ['\\', '|', '/'],
     [' ', '|', ' '],
     [' ', ' ', ' ']],
    [[' ', 'O', ' '],
     ['\\', '|', '/'],
     [' ', '|', ' '],
     ['/', ' ', ' ']],
    [[' ', 'O', ' '],
     ['\\', '|', '/'],
     [' ', '|', ' '],
     ['/', ' ', '\\']]];

fn print_man(i: usize) {
    assert!(i < MEN.len());
    let man = &MEN[i];
    for r in 0..man.len() {
        let row = &man[r];
        for c in 0..row.len() {
            print!("{}", row[c]);
        }
        println!();
    }
}    
                 

enum MysteryChar {
    Hidden(char),
    Found(char),
}
use MysteryChar::*;

struct Game {
    guesses: Vec<char>,
    word: Vec<MysteryChar>,
}

fn get_guess() -> char {
    print!("Guess a char: ");
    stdout().flush();
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.chars().next().unwrap()
}

fn main() {
    let mut rng = thread_rng();
    print_man(rng.gen_range(0, MEN.len() - 1));
    println!("{}", WORDS[rng.gen_range(0, WORDS.len())]);
    println!("{}", get_guess());
}
