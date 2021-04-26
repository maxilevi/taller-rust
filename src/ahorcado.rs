use std::fs;
use std::collections::HashMap;
use std::io::{self, BufRead};

struct AhorcadoEstado {
    current_word: Vec<char>,
    board: Vec<bool>,
    guesses: HashMap<char, u32>,
    intentos: u32
}

impl AhorcadoEstado {

    fn new(current_word: &String) -> Self {
        let mut freqs = HashMap::new();
        for letter in current_word.chars() {
            freqs.insert(letter, 0);
        }
        return Self {
            current_word: current_word.chars().collect(),
            board: vec![false; current_word.len()],
            guesses: freqs,
            intentos: 5
        };
    }

    fn won(self: &Self) -> bool {
        let mut won = true;
        for guessed in self.board.iter() {
            if !guessed {
                won = false;
                break;
            }
        }
        return won;
    }


    fn print(self: &Self) {
        let board_strings = self.board.iter().enumerate().map(|(i, x)| if *x { self.current_word[i].to_string() } else { "_".to_string() });
        let guesses_strings = self.guesses.iter().filter(|(_, x)| **x > 0).map(|(k, _)| k.to_string());

        println!("La palabra hasta el momento es: {}", board_strings.collect::<Vec<_>>().join(" "));
        println!("Adivinaste las siguientes letras: {}", guesses_strings.collect::<Vec<_>>().join(" "));
        println!("Te quedan {} intentos.", self.intentos);
    }

    fn guess(self: &mut Self, letter: char) -> bool {
        let prev = if self.guesses.contains_key(&letter) { self.guesses[&letter] } else { 0 };
        self.guesses.insert(letter,  prev + 1);
        for i in 0..self.current_word.len() {
            if !self.board[i] {
                if letter == self.current_word[i] {
                    self.board[i] = true;
                    return true;
                }
            }
        }
        self.intentos -= 1;
        if self.intentos == 0 {
            return false;
        }
        return true;
    }
}

pub fn jugar(path: &String) {

    let words = get_words(path);
    let mut current = 0;
    let mut state = AhorcadoEstado::new(&words[current]);
    println!("Bienvenido al ahorcado FIUBA!");
    while current < words.len() {
        while !state.won() {
            state.print();
            println!("Ingresa una letra:");

            let mut line = String::new();
            let stdin = io::stdin();
            stdin.lock().read_line(&mut line).unwrap();
            let char_vec: Vec<char> = line.chars().collect();
            if !state.guess(char_vec[0]) {
                println!("Te has quedado sin intentos");
                break;
            }
        }
        if state.won() {
            state.print();
            println!("Has adivinado la palabra!");
        }
        current += 1;
        if current < words.len() {
            state = AhorcadoEstado::new(&words[current]);
        }
    }
    println!("No hay mas palabras");
}

fn get_words(path: &String) -> Vec<String> {
    let contents = fs::read_to_string(path)
        .expect("Fallo al leer el archivo");

    let mut ans = Vec::new();
    for line in contents.lines() {
        ans.push(line.to_string());
    }
    return ans;
}
