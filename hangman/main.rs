use rand::Rng;
use std::io::{self, Write};

// hangman cli game
fn main() {
    println!("Welcome to Hangman!");

    // Array of words for the game
    let words = ["hangman", "rust", "programming", "openai", "game"];

    // Select a random word as the secret word
    let secret_word = select_random_word(&words);

    // Create a vector of underscores representing the guessed letters
    let mut guessed_letters = vec!['_'; secret_word.len()];

    let mut attempts = 6;

    loop {
        println!("\nAttempts remaining: {}", attempts);
        println!("Guessed letters: {}", guessed_letters.iter().collect::<String>());

        // Get user input for a letter or the full word guess
        let guess = get_input("Enter a letter or the full word: ");

        if guess.chars().count() == 1 {
            // User guessed a single letter
            let guessed_letter = guess.chars().next().unwrap();

            if secret_word.contains(guessed_letter) {
                // Correct guess: update the guessed letters
                for (index, letter) in secret_word.chars().enumerate() {
                    if letter == guessed_letter {
                        guessed_letters[index] = letter;
                    }
                }
            } else {
                // Incorrect guess: decrement attempts
                println!("Incorrect guess!");
                attempts -= 1;
            }
        } else if guess == secret_word {
            // User guessed the full word correctly
            println!("Congratulations, you won!");
            break;
        } else {
            // Incorrect guess for the full word: decrement attempts
            println!("Incorrect guess!");
            attempts -= 1;
        }

        if guessed_letters.iter().all(|&c| c != '_') {
            // All letters have been guessed correctly
            println!("Congratulations, you won!");
            break;
        }

        if attempts == 0 {
            // Player ran out of attempts
            println!("Sorry, you lost! The word was: {}", secret_word);
            break;
        }
    }
}

// Selects a random word from the given array
fn select_random_word<'a>(words: &'a [&str]) -> &'a str {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..words.len());
    words[index]
}

// Gets user input from the command line
fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_lowercase()
}
