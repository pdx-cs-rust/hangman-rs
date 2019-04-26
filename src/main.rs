#![allow(unused)]
// Copyright © 2019 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.


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

    fn new(mut rng: &mut ThreadRng) -> Self {
        let word = WORDS.choose(&mut rng).unwrap();
        let word: Vec<MysteryChar> =
            word.chars().map(|c| Hidden(c)).collect();
        Self {
            guesses: Vec::new(),
            word,
        }
    }

    fn is_won(&self) -> bool {
        for c in &self.word {
            if let &Hidden(_) = c {
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
        println!();
        let guesses_string: String = self.guesses.iter().collect();
        println!("{}", guesses_string);
        println!();
        print_man(self.guesses.len());
        println!();
        self.word.iter().for_each(|c| {
            match c {
                &Found(c) => print!("{} ", c),
                &Hidden(_) => print!("_ "),
            }
        });
        println!();
        println!();
    }

    fn update(&mut self, guess: char) {
        for c in &self.guesses {
            if *c == guess {
                println!("already guessed {}", c);
                return;
            }
        }
        let mut occurs = Vec::new();
        for i in 0..self.word.len() {
            match self.word[i] {
                Found(c0) if guess == c0 => {
                    println!("already guessed {}", c0);
                    return;
                },
                Hidden(c0) if guess == c0 => {
                    occurs.push(i);
                },
                _ => (),
            }
        }
        if occurs.is_empty() {
            self.guesses.push(guess);
            return;
        }
        for i in occurs {
            self.word[i] = Found(guess);
        }
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
