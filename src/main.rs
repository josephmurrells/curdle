use serde::{Deserialize, Serialize};
use std::fs;
use rand::Rng;
use std::io;
use colored::Colorize;
use colored::ColoredString;
use std::env;
use reqwest::Error;

#[derive(Deserialize, Serialize, Debug)]
struct WordJson {
    word: String,
    id: i16
}

struct CurdleGame {
    words: Vec<String>,
    tries: i16,
    answer: String,
    guess: String,
    previous_guesses: Vec<String>
}

impl CurdleGame {
    fn new() -> CurdleGame {
        Self { words: generate_words(), tries: 5, answer: String::new(), guess: String::new(), previous_guesses: Vec::new()}
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
                None => String::from("")
            }.to_lowercase();

            if validate_input(&self.guess, &self) {
                self.previous_guesses.push(String::from(&self.guess));
                break;
            }
        }
    }

    fn check_answer(&self) -> bool {
        let mut checked_word: Vec<ColoredString> = Vec::new();
        if self.guess == self.answer
        {
            println!("{}", self.guess.green());
            return true;
        }
        else
        {
            for (i, c) in self.guess.chars().enumerate() {
                if self.answer.chars().nth(i).unwrap() == c
                {
                    checked_word.push(String::from(c).to_uppercase().green());
                }
                else if self.answer.contains(c)
                {
                    checked_word.push(String::from(c).to_uppercase().yellow())
                }
                else
                {
                    checked_word.push(String::from(c).to_uppercase().red())
                }
            }
        }
        println!("{}{}{}{}{}", checked_word[0], checked_word[1], checked_word[2], checked_word[3], checked_word[4]);
        return false;
    }

    fn lose_life(&mut self){
        self.tries -= 1;
    }
}

#[tokio::main]
async fn main() {
    get_json().await.expect("Failed to get json");
    let mut new_game = CurdleGame::new();
    new_game.set_answer();
    println!("Guess the word!");
    while new_game.tries > 0
    {
        if new_game.previous_guesses.len() > 0
        {
            println!("Previously guessed: {:#?}", new_game.previous_guesses.join(", "));
            println!("----------------------------------------------------------------");
        }

        new_game.user_input();

        if new_game.check_answer()
        {
            println!("You win! You had {:#?} tries remaining", new_game.tries);
            break;
        }


        new_game.lose_life();

        if new_game.tries == 0 {
            println!("You lose! The correct word was {:#?}", new_game.answer);
            break;
        }

    }
}

fn generate_words() -> Vec<String> {
    let path = env::current_dir().expect("couldnt get path").join("words.json");
    let file =  fs::read_to_string(path).expect("Failed to open file");
    let words:Vec<WordJson> = serde_json::from_str(&file).expect("Failed to parse json");
    let words_array:Vec<String> = words.iter().map(|w| w.word.clone()).collect();
    return words_array;
}

fn validate_input(input:&String, curdle_game: &CurdleGame) -> bool {
    if !input.chars().all(|x| x.is_alphabetic()) {
        println!("{}", "Word must only contain letters".red());
        return false
    }
    else if input.chars().count() != 5
    {
        println!("Word must be 5 characters long, entered word was {:#?} long", input.chars().count());
        return false
    }
    else if !curdle_game.words.contains(input)
    {
        println!("Word not in curdle dictionary :(");
        return false
    }
    else if curdle_game.previous_guesses.contains(input)
    {
        println!("You already tried that word!");
        return false
    }
    else
    {
        return true
    }
}

async fn get_json() -> Result<(), Error> {
    let path = env::current_dir().expect("couldnt get path").join("words.json");

    if path.try_exists().unwrap(){
        return Ok(());
    }else{
        println!("Downloading dictionary file...");
        let response = reqwest::get("https://wordle-api.cyclic.app/words").await?;
        let words: Vec<WordJson> = response.json().await?;
        let path = env::current_dir().expect("couldnt get path").join("words.json");
        let json_string = serde_json::to_string(&words).unwrap();
        fs::write(path, json_string).expect("Unable to write file");
    }

    Ok(())
}

