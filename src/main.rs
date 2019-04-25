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

impl Game {

    fn new(rng: &mut ThreadRng) -> Self {
        let word = WORDS.choose(&mut rng).unwrap();
        let word: Vec<MysteryChar> =
            word.chars().map(|c| Hidden(c)).collect();
        Self {
            guesses: Vec::new(),
            word,
        }
    }

    fn is_won(&self) -> bool {
        for c in self.word {
            if let Hidden(_) = c {
                return false;
            }
        }
        true
    }

    fn is_lost(&self) -> bool {
        self.guesses.len() >= MEN.len()
    }

    fn is_finished(&self) -> bool {
        self.is_lost() || self.is_won()
    }

    fn print_state(&self) {
        let guesses_string: String = self.guesses.iter().collect();
        println!("{}", guesses_string);
        println!();
        print_man(self.guesses.len());
        println!();
        for c in self.word {
            match c {
                Found(c) => print!("{}", c),
                Hidden(_) => print!("_"),
            }
        }
        println!();
    }

    fn update(guess: char) {
        unimplemented!("need to play the game")
    }

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
    let mut game = Game::new(&mut rng);
    while !game.is_finished() {
        game.print_state();
        let guess = get_guess();
        game.update(guess);
    }
}
