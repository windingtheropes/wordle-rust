use std::io;
use std::fs;
use std::io::Read;
use colored::Colorize;

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
        println!("You will have 5 chances to guess the 5 letter word.");
    }
    fn start(&self) {
        self.intro();
        let mut guesses: Vec<WordGuess> = vec![];
        for _i in 0..5 {
            let word_guess: WordGuess = self.guess(&guesses);
            guesses.push(word_guess.clone());

            self.render(&guesses);
            if word_guess.is_correct == true {
                println!("Guessed in {}.", _i);
                break
            }
        }
    }
    fn render(&self, word_guess: &Vec<WordGuess>) {
        clear();
        for guess in word_guess {
            println!("{}", self.format_word(guess.guess.as_str(), &guess.char_state))
        }
    }
    fn guess_is_word(&self, guess: &str) -> bool {
        if get_words().contains(&guess.to_lowercase().to_owned()) {
            return true
        }
        false
    }
    // Ensure the guess is 5 characters then return it
    fn guess(&self, word_guess: &Vec<WordGuess>) -> WordGuess {
        let g = input();
        if g.len() == 5 && self.guess_is_word(&g.as_str()) {
            if g == self.word {
                WordGuess{
                    guess: g.clone(), 
                    char_state: self.get_guess_states(g.as_str()),
                    is_correct: true
                }
            }
            else {
                WordGuess{
                    guess: g.clone(), 
                    char_state: self.get_guess_states(g.as_str()),
                    is_correct: false
                }
            }
        }
        else {
            // Re-render the game to only show formatted guesses, if invalid input
            self.render(word_guess);
            self.guess(word_guess)
        }
    }
    // Return an array of 5 CharStates based on the guess in reference to the word
    fn get_guess_states(&self, guess: &str) -> Vec<CharState>{
        let correct_chars: Vec<char> = self.word.chars().map(|x| x).collect();
        let guess_chars: Vec<char> = guess.chars().map(|x| x).collect();
        
        let mut states: Vec<CharState> = vec![];

        // Iterate through 5 characters through the correct word and the guess, and compare based on the rules
        for i in 0..5 {
            let c_char = correct_chars[i];
            let g_char = guess_chars[i];
            if g_char == c_char {
                states.push(CharState::InPlace);
            }
            else if correct_chars.contains(&g_char) {
                states.push(CharState::InWord);
            }
            else {
                states.push(CharState::Not)
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
#[derive(Debug, Clone)]
enum CharState {
    Not,
    InWord,
    InPlace
}

fn get_words() -> Vec<String> {
    let mut word_file = fs::File::open("wordlist.txt").unwrap();
    let mut filebuf: String = String::new();
    let _ = word_file.read_to_string(&mut filebuf);
    let lines: Vec<String> = filebuf.lines().map(|x| x.to_owned()).collect();
    lines
}
fn random_word() -> String {
    let lines: Vec<String> = get_words();
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