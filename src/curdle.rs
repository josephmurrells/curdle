use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use rand::Rng;
use std::io;
use colored::Colorize;
use std::env;
use reqwest::Error;

#[derive(Deserialize, Serialize, Debug)]
struct WordJson {
    word: String,
    id: i16
}

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
                println!("\nYou win! You had {:#?} tries remaining", self.tries);
                break;
            }

            self.lose_life();

            if self.tries == 0 {
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

            if self.validate_input() {
                self.previous_guesses.push(String::from(&self.guess));
                break;
            }
        }
    }

    fn check_answer(&self) -> bool {
        if self.guess == self.answer
        {
            println!("{}", self.guess.green());
            return true;
        }
        else
        {
            for (i, c) in self.guess.chars().enumerate() {
                if self.answer.chars().nth(i).unwrap_or_default() == c
                {
                    print!("{}", String::from(c).to_uppercase().green());
                }
                else if self.answer.contains(c)
                {
                    print!("{}", String::from(c).to_uppercase().yellow());
                }
                else
                {
                    print!("{}", String::from(c).to_uppercase().red());
                }
            }
        }
        return false;
    }

    fn lose_life(&mut self){
        self.tries -= 1;
    }

    async fn generate_words(&mut self) {
        let path = env::current_dir().expect("couldnt get path").join("words.json");
        if !path.try_exists().unwrap()
        {
            CurdleGame::get_json(&path).await.expect("Failed to get json");
        }
        let file =  fs::read_to_string(path).expect("Failed to open file");
        let words:Vec<WordJson> = serde_json::from_str(&file).expect("Failed to parse json");
        let words_array:Vec<String> = words.iter().map(|w| w.word.clone()).collect();
        self.words = words_array;
    }

    async fn get_json(path:&PathBuf) -> Result<(), Error> {
        println!("Downloading dictionary file...");
        let response = reqwest::get("https://wordle-api.cyclic.app/words").await?;
        let words: Vec<WordJson> = response.json().await?;
        let json_string = serde_json::to_string(&words).unwrap();
        fs::write(path, json_string).expect("Unable to write file");
        println!("Dictionary file successfully downloaded!");
        println!("----------------------------------------------------------------");

        Ok(())
    }

    fn validate_input(&self) -> bool {
        return self.validate_alphabetic() &&
        self.validate_word_length() &&
        self.validate_in_dictionary() &&
        self.validate_previously_guessed();
    }

    fn validate_previously_guessed(&self) -> bool {
        if self.previous_guesses.contains(&self.guess)
        {
            println!("You already tried that word!");
            return false
        }else {
            return true
        }
    }

    fn validate_in_dictionary(&self) -> bool {
        if self.words.contains(&self.guess)
        {
            return true
        }else {
            println!("Word not in curdle dictionary :(");
            return false
        }
    }

    fn validate_alphabetic(&self) -> bool {
        if self.guess.chars().all(|x| x.is_alphabetic()) {
            return true
        }else {
            println!("{}", "Word must only contain letters".red());
            return false
        }
    }

    fn validate_word_length(&self) -> bool {
        if self.guess.chars().count() == 5 {
            return true
        }else {
            println!("Word must be 5 characters long, entered word was {:#?} long", self.guess.chars().count());
            return false
        }
    }
}
