use rand::Rng;
use std::io;
use colored::Colorize;

mod json;
mod validation;

pub struct CurdleGame {
    words: Vec<String>,
    tries: i16,
    answer: String,
    guess: String,
    previous_guesses: Vec<String>
}

impl CurdleGame {
    pub fn new(tries:i16) -> CurdleGame {
        Self {
            words: Vec::new(),
            tries: tries,
            answer: String::new(),
            guess: String::new(),
            previous_guesses: Vec::new()
        }
    }

    pub async fn start_game(&mut self) {
        self.generate_words().await;
        self.set_answer();
        println!("Guess the word!");
        println!("----------------------------------------------------------------");
        while self.tries > 0
        {
            if self.previous_guesses.len() > 0
            {
                println!("\nPreviously guessed: {:#?}", self.previous_guesses.join(", "));
                println!("----------------------------------------------------------------");
            }

            self.user_input();

            if self.check_answer()
            {
                println!("\n----------------------------------------------------------------");
                println!("\nYou win! You had {:#?} tries remaining", self.tries);
                break;
            }

            self.lose_life();

            if self.tries == 0 {
                println!("\n----------------------------------------------------------------");
                println!("\nYou lose! The correct word was {:#?}", self.answer);
                break;
            }

        }
    }

    fn set_answer(&mut self) {
        self.answer = self.words[rand::thread_rng().gen_range(0..self.words.len())].clone();
    }

    fn user_input(&mut self) {
        loop
        {
            if self.guess.len() > 0 { self.guess = String::new(); }

            io::stdin()
                .read_line(&mut self.guess)
                .expect("Failed to read line");

            self.guess = match self.guess.strip_suffix("\r\n") {
                Some(string) => String::from(string),
                None => continue
            }.to_lowercase();

            if validation::validate_input(&self) {
                self.previous_guesses.push(String::from(&self.guess));
                break;
            }
        }
    }

    fn check_answer(&self) -> bool {

        let mut non_matching_chars = self.get_non_matching_chars();

        if self.guess == self.answer
        {
            println!("{}", self.guess.to_uppercase().green());
            return true;
        }

        for (i, c) in self.guess.chars().enumerate() {
            if self.answer.chars().nth(i).unwrap_or_default() == c
            {
                print!("{}", String::from(c).to_uppercase().green());
            }
            else if non_matching_chars.contains(&c)
            {
                print!("{}", String::from(c).to_uppercase().yellow());
                non_matching_chars.retain(|&char| char != c)
            }
            else
            {
                print!("{}", String::from(c).to_uppercase().red());
            }
        }

        return false;
    }

    fn get_non_matching_chars(&self) -> Vec<char> {
        let mut non_matching_chars = Vec::new();

        for (i, c) in self.guess.chars().enumerate() {
            let answer_char = self.answer.chars().nth(i).unwrap_or_default();

            if answer_char != c
            {
                non_matching_chars.push(answer_char)
            }
        }

        return non_matching_chars;
    }

    fn lose_life(&mut self){
        self.tries -= 1;
    }

    async fn generate_words(&mut self) {
        self.words = json::words_from_json().await;
    }
}
