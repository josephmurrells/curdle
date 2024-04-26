use super::CurdleGame;
use super::Colorize;

pub fn validate_input(curdle_game: &CurdleGame) -> bool {
    return validate_alphabetic(curdle_game) &&
    validate_word_length(curdle_game) &&
    validate_in_dictionary(curdle_game) &&
    validate_previously_guessed(curdle_game);
}

fn validate_previously_guessed(curdle_game: &CurdleGame) -> bool {
    if curdle_game.previous_guesses.contains(&curdle_game.guess)
    {
        print_warning_string("You already tried that word!");
        return false
    }else {
        return true
    }
}

fn validate_in_dictionary(curdle_game: &CurdleGame) -> bool {
    if curdle_game.words.contains(&curdle_game.guess)
    {
        return true
    }else {
        print_warning_string("Word not in curdle dictionary");
        return false
    }
}

fn validate_alphabetic(curdle_game: &CurdleGame) -> bool {
    if curdle_game.guess.chars().all(|x| x.is_alphabetic()) {
        return true
    }else {
        print_warning_string("Word must only contain letters");
        return false
    }
}

fn validate_word_length(curdle_game: &CurdleGame) -> bool {
    if curdle_game.guess.chars().count() == 5 {
        return true
    }else {
        print_warning_string("Word must be 5 characters long");
        return false
    }
}

fn print_warning_string(warning_string: &str) {
    println!("{}", warning_string.red());
}
