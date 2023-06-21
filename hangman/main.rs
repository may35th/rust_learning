use rand::Rng;
use std::io::{self, Write};

fn main() {
    println!("Welcome to Hangman!");

    let words = ["hangman", "rust", "programming", "openai", "game"];
    let secret_word = select_random_word(&words);
    let mut guessed_letters = vec!['_'; secret_word.len()];

    let mut attempts = 6;

    loop {
        println!("\nAttempts remaining: {}", attempts);
        println!("Guessed letters: {}", guessed_letters.iter().collect::<String>());

        let guess = get_input("Enter a letter or the full word: ");

        if guess.chars().count() == 1 {
            let guessed_letter = guess.chars().next().unwrap();

            if secret_word.contains(guessed_letter) {
                for (index, letter) in secret_word.chars().enumerate() {
                    if letter == guessed_letter {
                        guessed_letters[index] = letter;
                    }
                }
            } else {
                println!("Incorrect guess!");
                attempts -= 1;
            }
        } else if guess == secret_word {
            println!("Congratulations, you won!");
            break;
        } else {
            println!("Incorrect guess!");
            attempts -= 1;
        }

        if guessed_letters.iter().all(|&c| c != '_') {
            println!("Congratulations, you won!");
            break;
        }

        if attempts == 0 {
            println!("Sorry, you lost! The word was: {}", secret_word);
            break;
        }
    }
}

fn select_random_word<'a>(words: &'a [&str]) -> &'a str {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..words.len());
    words[index]
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_lowercase()
}
