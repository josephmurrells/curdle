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

#[cfg(test)]
mod tests {
    use super::*;

    fn build_game(guess: &str, words: Vec<&str>, previous_guesses: Vec<&str>) -> CurdleGame {
        CurdleGame {
            words: words.into_iter().map(String::from).collect(),
            tries: 6,
            answer: String::from("dummy"),
            guess: String::from(guess),
            previous_guesses: previous_guesses.into_iter().map(String::from).collect(),
        }
    }

    #[test]
    fn validate_alphabetic_returns_true_for_letters_only() {
        let game = build_game("piano", vec!["piano"], vec![]);
        assert!(validate_alphabetic(&game));
    }

    #[test]
    fn validate_alphabetic_returns_false_for_non_letters() {
        let game = build_game("pi4no", vec!["pi4no"], vec![]);
        assert!(!validate_alphabetic(&game));
    }

    #[test]
    fn validate_word_length_returns_true_for_five_characters() {
        let game = build_game("piano", vec!["piano"], vec![]);
        assert!(validate_word_length(&game));
    }

    #[test]
    fn validate_word_length_returns_false_for_non_five_characters() {
        let game = build_game("cat", vec!["cat"], vec![]);
        assert!(!validate_word_length(&game));
    }

    #[test]
    fn validate_in_dictionary_returns_true_when_word_exists() {
        let game = build_game("piano", vec!["piano", "proud"], vec![]);
        assert!(validate_in_dictionary(&game));
    }

    #[test]
    fn validate_in_dictionary_returns_false_when_word_missing() {
        let game = build_game("piano", vec!["proud"], vec![]);
        assert!(!validate_in_dictionary(&game));
    }

    #[test]
    fn validate_previously_guessed_returns_true_for_new_guess() {
        let game = build_game("piano", vec!["piano"], vec!["proud"]);
        assert!(validate_previously_guessed(&game));
    }

    #[test]
    fn validate_previously_guessed_returns_false_for_repeat_guess() {
        let game = build_game("piano", vec!["piano"], vec!["piano", "proud"]);
        assert!(!validate_previously_guessed(&game));
    }

    #[test]
    fn validate_input_returns_true_for_valid_new_dictionary_word() {
        let game = build_game("piano", vec!["piano", "proud"], vec!["proud"]);
        assert!(validate_input(&game));
    }

    #[test]
    fn validate_input_returns_false_when_any_validation_fails() {
        let not_alphabetic = build_game("pi4no", vec!["pi4no"], vec![]);
        assert!(!validate_input(&not_alphabetic));

        let wrong_length = build_game("tool", vec!["tool"], vec![]);
        assert!(!validate_input(&wrong_length));

        let not_in_dictionary = build_game("piano", vec!["proud"], vec![]);
        assert!(!validate_input(&not_in_dictionary));

        let previously_guessed = build_game("piano", vec!["piano"], vec!["piano"]);
        assert!(!validate_input(&previously_guessed));
    }
}
