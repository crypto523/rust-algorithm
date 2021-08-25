use std::collections::HashMap;

const UNKNOWN_CHARACTER: &str = "........";

pub fn morse_code(message: &str) -> String {
    let dictionary = morse_dictionary();
    message
        .chars()
        .into_iter()
        .map(|char| char.to_uppercase().to_string())
        .map(|letter| dictionary.get(letter.as_str()))
        .map(|option| option.unwrap_or(&UNKNOWN_CHARACTER).to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

// Declaritive macro for creating readable map declarations, for more info see https://doc.rust-lang.org/book/ch19-06-macros.html
macro_rules! map {
    ($($key:expr => $value:expr),* $(,)?) => {
        std::iter::Iterator::collect(std::array::IntoIter::new([$(($key, $value),)*]))
    };
}

fn morse_dictionary() -> HashMap<&'static str, &'static str> {
    map! {
        "A" => ".-",      "B" => "-...",    "C" => "-.-.",
        "D" => "-..",     "E" => ".",       "F" => "..-.",
        "G" => "--.",     "H" => "....",    "I" => "..",
        "J" => ".---",    "K" => "-.-",     "L" => ".-..",
        "M" => "--",      "N" => "-.",      "O" => "---",
        "P" => ".--.",    "Q" => "--.-",    "R" => ".-.",
        "S" => "...",     "T" => "-",       "U" => "..-",
        "V" => "...-",    "W" => ".--",     "X" => "-..-",
        "Y" => "-.--",    "Z" => "--..",

        "1" => ".----",   "2" => "..---",   "3" => "...--",
        "4" => "....-",   "5" => ".....",   "6" => "-....",
        "7" => "--...",   "8" => "---..",   "9" => "----.",
        "0" => "-----",

        "&" => ".-...",   "@" => ".--.-.",  ":" => "---...",
        "," => "--..--",  "." => ".-.-.-",  "'" => ".----.",
        "\"" => ".-..-.", "?" => "..--..",  "/" => "-..-.",
        "=" => "-...-",   "+" => ".-.-.",   "-" => "-....-",
        "(" => "-.--.",   ")" => "-.--.-",  " " => "/",
        "!" => "-.-.--",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_only_letters() {
        let message = "Hello Morse";
        let cipher = morse_code(message);
        assert_eq!(
            cipher,
            ".... . .-.. .-.. --- / -- --- .-. ... .".to_string()
        )
    }

    #[test]
    fn encrypt_letters_and_special_characters() {
        let message = "What's a great day!";
        let cipher = morse_code(message);
        assert_eq!(
            cipher,
            ".-- .... .- - .----. ... / .- / --. .-. . .- - / -.. .- -.-- -.-.--".to_string()
        )
    }

    #[test]
    fn encrypt_message_with_unsupported_character() {
        let message = "Error?? {}";
        let cipher = morse_code(message);
        assert_eq!(
            cipher,
            ". .-. .-. --- .-. ..--.. ..--.. / ........ ........".to_string()
        )
    }
}
