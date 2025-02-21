use std::collections::HashMap;
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

        // Initial render the blank screen of 6 empty places
        pause();
        self.render(&guesses);

        if || -> bool {
            for _i in 0..6 {
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
    
    fn get_used_alphabet(&self, word_guess: &Vec<WordGuess>) -> String {
        let alpha: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().map(|x| x).collect();
        let mut states: Vec<CharState> = alpha.iter().map(|_| Unused).collect();
        
        for guess in word_guess {
            for i in 0..5 {
                let chars: Vec<char> = guess.guess.chars().map(|x| x).collect();
                let c = chars[i];
                let s = &guess.char_state[i];

                if alpha.contains(&c) == true {
                    let char_index = alpha.iter().position(|p| p == &c).unwrap();

                    // In*,Not overwrite Unused and Not, Green overwrites Yellow
                    if states[char_index] == Unused || (states[char_index] == InWord && *s == InPlace) {
                        states[char_index] = s.clone();
                    }
                }
            }
        }
        let alpha_string: String = alpha.iter().collect();
        return self.colour_format(alpha_string.as_str(), &states)
    }

    // Render the board
    fn render(&self, word_guess: &Vec<WordGuess>) {
        clear();
        let max_index = *&word_guess.len() as i32;
        for i in 0..6 {
            if i > max_index - 1 {
                println!("_____");
            }
            else {
                let guess = &word_guess[i as usize];
                println!("{}", self.colour_format(guess.guess.as_str(), &guess.char_state))
            }
        }

        // Render the alphabet below
        
        println!("\n{}", self.get_used_alphabet(&word_guess))
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
    
    fn word_guess_from_str(&self, guess: &str) -> WordGuess {
        let mut g: WordGuess = WordGuess {
            guess: guess.to_string(),
            char_state: self.get_guess_states(guess),
            is_correct: false,
        };
        if guess == self.word {
            g.is_correct = true;
        }
        return g.clone();
    }
    // Ensure the guess is 5 characters then return it
    fn guess(&self, guesses: &Vec<WordGuess>) -> WordGuess {
        let g: String = input().to_lowercase();
        if g.len() == 5 && self.guess_is_word(&g.as_str()) && !self.is_already_guessed(&g, &guesses) {
            return self.word_guess_from_str(&g);
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
        let mut present_chars: HashMap<char, i32> = HashMap::new();
        
        // Count characters present in word
        for char in correct_chars.as_slice() {
            match present_chars.get_mut(char) {
                Some(c) => *c+= 1,
                None => { present_chars.insert(*char, 1); }
            }
        }

        // Two passes of checking, check for InPlace before InWord, in order to not light up extra letters

        // Search by characters in the correct word, first pass only for exactly correct (in place) characters
        for i in 0..5 {
            let g_char = &guess_chars[i];
            // First pass try to find exact matches
            if *g_char == *&correct_chars[i] && *present_chars.get_mut(g_char).unwrap() > 0 {
                states[i] = InPlace;
                // We know this can't be None
                *present_chars.get_mut(g_char).unwrap() -= 1;
            }
        }

        // Second pass try to find letters in word, not in place
        for i in 0..5 {
            let g_char = &guess_chars[i];
            if present_chars.contains_key(g_char) {
                // We now know, per above, this can't be None
                if *present_chars.get_mut(g_char).unwrap() > 0 {
                    states[i] = InWord;
                    *present_chars.get_mut(g_char).unwrap() -= 1;
                }
            }
        }
        states
    }
    // Print the colour formatted guess
    fn colour_format(&self, uf_string: &str, states: &Vec<CharState>) -> String {
        let mut formatted: String = String::new();
        let guess_chars: Vec<char> = uf_string.chars().map(|x| x).collect();
        for i in 0..guess_chars.len() {
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
                CharState::Unused => {
                    guess_chars[i].to_string()
                },
            };
            formatted = format!("{}{}", formatted, color_char);
        }
        formatted
    }
}
#[derive(Debug, Clone, PartialEq)]
enum CharState {
    Unused,
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

fn pause() {
    println!("Press enter to continue...");
    std::io::stdin().read(&mut [0]).unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_states() {
        let game: Game = Game {
            word: String::from("peace")
        };
        assert_eq!(game.get_guess_states("peice"), vec![InPlace, InPlace, Not, InPlace, InPlace]);

        let game: Game = Game {
            word: String::from("slime")
        };
        assert_eq!(game.get_guess_states("peice"), vec![Not, Not, InPlace, Not, InPlace]);
    }
}