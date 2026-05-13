use std::io;
use colored::Colorize;
use random_word::Lang;

mod validation;

pub struct CurdleGame {
    tries: i16,
    answer: String,
    guess: String,
    previous_guesses: Vec<String>
}

impl CurdleGame {
    pub fn new(tries:i16) -> CurdleGame {
        Self {
            tries,
            answer: String::new(),
            guess: String::new(),
            previous_guesses: Vec::new()
        }
    }

    pub async fn start_game(&mut self) {
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
                println!("----------------------------------------------------------------");
                println!("\nYou win! You had {:#?} tries remaining.", self.tries);
                break;
            }

            self.lose_life();

            if self.tries == 0 {
                println!("----------------------------------------------------------------");
                println!("\nYou lose! The correct word was {:#?}.", self.answer);
                break;
            }

        }
    }

    fn set_answer(&mut self) {
        self.answer = random_word::get_len(5, Lang::En).unwrap_or_default().to_string();
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
}

    #[cfg(test)]
    mod tests {
        use super::CurdleGame;

        fn build_game(tries: i16, answer: &str, guess: &str, previous: Vec<&str>) -> CurdleGame {
            CurdleGame {
                tries,
                answer: String::from(answer),
                guess: String::from(guess),
                previous_guesses: previous.into_iter().map(String::from).collect(),
            }
        }

        #[test]
        fn new_initializes_default_state() {
            let game = CurdleGame::new(6);

            assert_eq!(game.tries, 6);
            assert!(game.answer.is_empty());
            assert!(game.guess.is_empty());
            assert!(game.previous_guesses.is_empty());
        }

        #[test]
        fn sets_answer_generates_word() {
            let mut game = build_game(6, "", "", vec!["piano", "proud", "petty"]);

            game.set_answer();

            assert!(!game.answer.is_empty());
        }

        #[test]
        fn lose_life_decrements_tries() {
            let mut game = build_game(3, "petty", "piano", vec![]);

            game.lose_life();

            assert_eq!(game.tries, 2);
        }

        #[test]
        fn get_non_matching_chars_collects_unmatched_answer_chars() {
            let game = build_game(6, "petty", "piano", vec![]);

            let non_matching = game.get_non_matching_chars();

            assert_eq!(non_matching, vec!['e', 't', 't', 'y']);
        }

        #[test]
        fn check_answer_returns_true_for_exact_match() {
            let game = build_game(6, "petty", "petty", vec![]);

            assert!(game.check_answer());
        }

        #[test]
        fn check_answer_returns_false_for_non_match() {
            let game = build_game(6, "petty", "piano", vec![]);

            assert!(!game.check_answer());
        }
    }
