use std::io;
use std::fs;
use std::io::Read;


// Max guesses 5
struct Game {
    word: String,
    guess_count: u8,
    guesses: Vec<String>
}
impl Game {
    fn start(&self) {
        println!("Welcome to non NYT affiliated word guessing game.");
        println!("You will have 5 chances to guess the 5 letter word.")
    }
    // Ensure the guess is 5 characters then return it
    fn guess(&self) -> String {
        let g = input();
        if g.len() == 5 {
            g
        }
        else {
            self.guess()
        }
    }
    // Return an array of 5 CharStates based on the guess in reference to the word
    fn get_guess_state(&self, guess: &str) -> Vec<CharState>{
        let correct_chars: Vec<char> = self.word.chars().map(|x| x).collect();
        let guess_chars: Vec<char> = guess.chars().map(|x| x).collect();
        
        let mut states: Vec<CharState> = vec![];

        // Iterate through 5 characters through the correct word and the guess, and compare based on the rules
        for i in 0..5 {
            let c_char = correct_chars[i];
            let g_char = guess_chars[i];
            if  g_char {
                states.push(CharState::InPlace)
            }
            else if 
        }
        states
    }
}
enum CharState {
    Not,
    InWord,
    InPlace
}
fn random_word() -> String {
    let mut word_file = fs::File::open("wordlist.txt").unwrap();
    let mut filebuf: String = String::new();
    let _ = word_file.read_to_string(&mut filebuf);
    let lines: Vec<&str> = filebuf.lines().map(|x| x).collect();
    let randi = rand::random_range(0..lines.len());
    String::from(lines[randi])
}   
fn new_game() -> Game {
    let initial_guesses: Vec<String> = vec![];
    Game {
        word:random_word(),
        guess_count:0,
        guesses: initial_guesses
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