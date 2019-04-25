#![allow(unused)]

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

fn main() {
    print_man(MEN.len() - 1);
    println!("{}", WORDS[1]);
}
