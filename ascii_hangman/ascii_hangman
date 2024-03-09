/*  Hangman game written in Rust
Using Ascii art for the gallows
Fred Sheehan  2/3/2024  v.1.0
*/

// Importing the necessary libraries
use rand::seq::SliceRandom;
use std::io::{self, Write};

// All Rust programmes have to have a main function
fn main() {
    /*
    Print the welcome message to console the \n is
    used to create a new line before the message \t
    would be used if we wanted to create a tab spacing
    */
    println!("\n Welcome to Hangman!");

    /*  Wordlist used for our hangman game stored in a
    vector, this is similar to an array, but more flexible
    */
    let words = vec![
        "apple",
        "banana",
        "orange",
        "grape",
        "peach",
        "strawberry",
        "potato",
        "cucumber",
        "carrot",
        "broccoli",
        "asparagus",
        "spinach",
        "lettuce",
        "cabbage",
        "onion",
        "garlic",
        "celery",
        "mushroom",
        "pineapple",
        "mango",
        "kiwi",
        "plum",
        "apricot",
        "pear",
        "lemon",
        "lime",
        "avocado",
        "coconut",
        "walnut",
        "almond",
        "peanut",
    ];

    // Maximum number of incorrect guesses allowed
    let max_guesses = 15;

    // Choose a random word using the rand library
    // to first create a random number
    let mut rng = rand::thread_rng();

    // And then use that random number to choose the word from the list
    // We then unwrap it to get the individual characters in it
    let word = words.choose(&mut rng).unwrap();

    // Take the random word length and create a vector of
    // underscores the same length to display to the user
    // as they guess the word
    let mut guessed_letters = vec!['_'; word.len()];

    // Initialise a counter for the number of incorrect tries taken
    let mut incorrect_guesses = 0;

    // Create a new Vector to store the users guesssed characters
    let mut guessed_chars = Vec::new();

    // create a variable to count how many parts are displayed in our gallows
    // this allows us to end the game earlier than max guesses allows
    let mut parts_displayed = 0;

    // Create a variable to hold a boolean value to check if the
    // game is over and use this to control our game loop
    let mut game_over = false;

    // Create our starting empty gallows image stored as a tuple
    let mut gallows = ["  ____", " |", " |", " |", " |", "_|______"];

    /* While our boolean value is false, keep the game running using a while
       loop. This is the actual game loop
    */
    while !game_over {
        // Print 'Current word: ' with any correctly guessed letters
        println!(
            "Current Word: {}",
            guessed_letters.iter().collect::<String>()
        );

        // Print a line of the characters that have been guessed already
        println!("Guessed Characters: {:?}", guessed_chars);

        // Print the number of incorrect guesses remaining
        println!(
            "Incorrect Guesses Remaining: {}",
            // By subtracting incorrect_guesses from max_guesses
            max_guesses - incorrect_guesses
        );

        // Print the empty gallows tuple at the start of the game
        for line in &gallows {
            println!("{}", line);
        }

        /* If the guessed letters vector still contains any underscores
           then the game is still ongoing
        */
        if guessed_letters.iter().any(|&x| x == '_') {
            println!("Keep guessing!");
        }

        // If the number of incorrect_guesses is equal to the max_guesses
        if incorrect_guesses == max_guesses {
            println!("You ran out of guesses! The word was: {}", word);

            // Set the game over boolean to true
            game_over = true;

            // And ask if the player wants another game using the
            // game_replay function below
            game_replay();
        }

        // If the guessed letters vector contains no underscores
        // then the player has guessed the correct word

        if guessed_letters.iter().all(|&x| x != '_') {
            println!("Congratulations, you guessed the word: {}", word);

            // Set the game over boolean to true
            game_over = true;

            // And ask if the player wants another game
            game_replay();
        }

        // Ask the user to input a guessed letter
        print!("Enter a letter: ");

        // Flush the print buffer to ensure the message is displayed
        io::stdout().flush().unwrap();

        // Create a new string to store the user input
        let mut guess = String::new();

        // Read the user input with the standard input and store it in the
        // guess variable as characters (unwrap it)
        io::stdin().read_line(&mut guess).unwrap();

        // Check we have a valid input from the user using the match function
        let guess = match guess.trim().chars().next() {
            Some(c) => c,
            None => {
                println!("Invalid input!");
                continue;
            }
        };

        // Check if the guessed letter is already in the guessed_chars vector
        if guessed_chars.contains(&guess) {
            // If it is, print a message to the user and continue the loop
            println!("You already guessed that letter!");
            continue;
        }

        // Add the guessed letter to the guessed_chars vector
        guessed_chars.push(guess);

        // Then Check if the guessed letter is in the word, if it is,
        // update the guessed_letters vector with the correct letter
        let mut found_char = false;
        for (i, c) in word.chars().enumerate() {
            if c == guess {
                guessed_letters[i] = c;
                found_char = true;
            }
        }

        // If the guessed letter is not in the word
        if !found_char {
            // Increment the incorrect_guesses counter
            incorrect_guesses += 1;

            // Call the update_gallows function to update the gallows
            // based on the number of incorrect guesses

            update_gallows(&mut gallows, &mut parts_displayed, incorrect_guesses);

            // Increment the parts_displayed counter
            parts_displayed += 1;
        }

        // If the parts_displayed counter reaches 8, the gallows is complete
        // and the player has lost the game

        if parts_displayed == 8 {
            println!("You were hanged! The word was: {}", word);

            // Set the game over boolean to true
            game_over = true;

            // And ask if the player wants another game
            game_replay();
        }
    }
}

/*
Function using the match function to update the gallows based on the number
of incorrect guesses, and _parts_displayed of the gallows shown. Note the
underscore prepending the _parts_displayed variable used to indicate to the rust compiler that the parameter is not called directly, else it would throw a warning about the parameter not being used. This is a common convention in Rust.
*/
fn update_gallows(gallows: &mut [&str; 6], _parts_displayed: &mut usize, incorrect_guesses: usize) {
    match incorrect_guesses {
        1 => gallows[1] = " |    |",
        2 => gallows[2] = " |    O",
        3 => gallows[3] = " |    |",
        4 => gallows[3] = " |   /|",
        5 => gallows[3] = " |   /|\\",
        6 => gallows[4] = " |    |",
        7 => gallows[4] = " |   / \\",
        _ => {}
    }
}

// Function that asks the user if they want to play again
fn game_replay() {
    print!("Play again? (y/n): ");

    // Flush the output to the console
    io::stdout().flush().unwrap();

    // Create a mutable string to store the users response
    let mut response = String::new();

    // Using the stdin function from the io module, read the users input
    io::stdin().read_line(&mut response).unwrap();

    // If the user enters "y" (trim removes whitespace from user input)
    // then call the main function again to start a new game.
    if response.trim() == "y" {
        main();
    } else {
        // Otherwise print "Goodbye!" and exit the program
        println!("Goodbye!");
        std::process::exit(0);
    }
}
