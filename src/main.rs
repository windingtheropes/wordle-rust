use std::io;
use std::fs;
use std::io::Read;
use colored::Colorize;
use CharState::*;
// Max guesses 5
#[derive(Clone)]
struct WordGuess {
    guess: String,
    char_state: Vec<CharState>,
    is_correct: bool,
}
struct Game {
    word: String
}
impl Game {
    fn intro(&self) {
        clear();
        println!("Welcome to non NYT affiliated word guessing game.");
        println!("You will have 6 chances to guess the 5 letter word.");
    }
    fn start(&self) {
        self.intro();
        let mut guesses: Vec<WordGuess> = vec![];
        if || -> bool {
            for _i in 0..5 {
                let word_guess: WordGuess = self.guess(&guesses);
                guesses.push(word_guess.clone());
    
                self.render(&guesses);
                if word_guess.is_correct == true {
                    println!("Guessed in {}.", _i+1);
                    return true
                }
            };
            false
        }() == false {
            println!("{}", self.word)
        }
    }
    fn render(&self, word_guess: &Vec<WordGuess>) {
        clear();
        let max_index = *&word_guess.len() as i32;
        for i in 0..6 {
            if i > max_index - 1 {
                println!("_____");
            }
            else {
                let guess = &word_guess[i as usize];
                println!("{}", self.format_word(guess.guess.as_str(), &guess.char_state))
            }
        }
    }
    fn guess_is_word(&self, guess: &str) -> bool {
        if get_lines("5letter.txt").contains(&guess.to_lowercase().to_owned()) {
            return true
        }
        false
    }
    
    fn is_already_guessed(&self, guess: &str, guesses: &Vec<WordGuess>) -> bool {
        for word_guess in guesses {
            if word_guess.guess == guess {
                return true
            }
        }
        false
    }
    
    // Ensure the guess is 5 characters then return it
    fn guess(&self, guesses: &Vec<WordGuess>) -> WordGuess {
        let g: String = input().to_lowercase();
        if g.len() == 5 && self.guess_is_word(&g.as_str()) && !self.is_already_guessed(&g, &guesses) {
            if g == self.word {
                return WordGuess{
                    guess: g.clone(), 
                    char_state: self.get_guess_states(g.as_str()),
                    is_correct: true
                }
            }
            else {
                return WordGuess{
                    guess: g.clone(), 
                    char_state: self.get_guess_states(g.as_str()),
                    is_correct: false
                }
            }
        }
        else {
            // Re-render the game to only show formatted guesses, if invalid input
            self.render(&guesses);
            return self.guess(&guesses)
        }
    }
    // Return an array of 5 CharStates based on the guess in reference to the word
    fn get_guess_states(&self, guess: &str) -> Vec<CharState> {
        // Character vectors
        let correct_chars: Vec<char> = self.word.chars().map(|x| x).collect();
        let guess_chars: Vec<char> = guess.chars().map(|x| x).collect();
        
        // Default states vector of Not
        let mut states: Vec<CharState> = vec![Not, Not, Not, Not, Not];

        // Search by characters in the correct word
        for char in correct_chars.as_slice() {
            let chars_in_correct: Vec<&char> = correct_chars.iter().filter(|c| **c == *char).collect();
            let mut count = chars_in_correct.len() as i32;

            // First pass try to find exact matches
            for i in 0..5 {
                if *&guess_chars[i] == *&correct_chars[i] && *&states[i] == Not && count > 0 {
                    states[i] = InPlace;
                    count -= 1;
                }
            }

            // Second pass to try and find indirect matches, or Not
            for i in 0..5 {
                if *&guess_chars[i] == *char && *&states[i] == Not && count > 0 {
                    states[i] = InWord;
                    count -= 1;
                }
            }
        }
        states
    }
    // Print the colour formatted word
    fn format_word(&self, guess: &str, states: &Vec<CharState>) -> String {
        let mut formatted: String = String::new();
        let guess_chars: Vec<char> = guess.chars().map(|x| x).collect();
        for i in 0..5 {
            let color_char = match states[i] {
                CharState::InPlace => {
                    guess_chars[i].to_string().green().to_string()
                },
                CharState::InWord => {
                    guess_chars[i].to_string().yellow().to_string()
                },
                CharState::Not => {
                    guess_chars[i].to_string().truecolor(150, 150, 150).to_string()
                },
            };
            formatted = format!("{}{}", formatted, color_char);
        }
        formatted
    }
}
#[derive(Debug, Clone, PartialEq)]
enum CharState {
    Not,
    InWord,
    InPlace
}

fn get_lines(path: &str) -> Vec<String> {
    let mut word_file = fs::File::open(path).unwrap();
    let mut filebuf: String = String::new();
    let _ = word_file.read_to_string(&mut filebuf);
    let lines: Vec<String> = filebuf.lines().map(|x| x.to_owned()).collect();
    lines
}
fn random_word() -> String {
    let lines: Vec<String> = get_lines("5letternoplurals.txt");
    let randi = rand::random_range(0..lines.len());
    lines[randi].clone()
}   
fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

fn new_game() -> Game {
    Game {
        word:random_word()
    }
}

fn main() {
    let game = new_game();  
    game.start();
}

fn input() -> String {
    let mut buf: String = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    String::from(buf.trim())
}