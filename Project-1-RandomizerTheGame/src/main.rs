use std::io::{self, Write};
use rand::Rng;

fn main() {

    // enjoy this simple game :) => last stable version is [v1.2.0]
    start_greeting();

    let random_number : i16 = rand::rng().random_range(0..=100);
    let mut second_chance : bool = false;
    let mut tries : u8 = 10;

    loop {

        print!("Please, enter here your guess: ");
        match io::stdout().flush() {
            Ok(_) => {},
            Err(_) => {
                println!("Sorry, failed to flush the output! :( [please, try again]");
                continue;
            }
        };

        let mut guess = String::new();
        match std::io::stdin().read_line(&mut guess) {
            Ok(_) => {},
            Err(_) => {
                println!("Sorry, failed to read the line! :( [please, try again]");
                continue;
            }
        };

        let guess: f64 = match guess.trim().parse::<f64>() {
            Ok(num) => {
                if num > 100.0 && !num.is_infinite() {
                    println!("Your guess is out of my thought random number! Not valid!");
                    tries -= 1;
                    continue;
                } else if num.is_infinite() {
                    println!("Your guess is a valid number! Error of overflow was avoided!");
                    tries -= 1;
                    continue;
                } else {
                    num
                }
            },
            Err(_) => {
                println!("Sorry, I expected a number, but you gave me something else :( [please, try again]");
                continue;
            }
        };

        let guess : i16 = check_negative_guess(check_float_guess(guess)) as i16;
        let diff : i16 = guess - random_number;
        let diff_abs : i16 = diff.abs();

        if diff_abs >= 50 {
            if diff > 0 {
                println!("Your guess is way too far! Try to get lower.");
            } else {
                println!("Your guess is way too far! Try to get higher.");
            }
            tries -= 1;
        } else if 30 <= diff_abs && diff_abs < 50 {
            if diff > 0 {
                println!("Your guess is too far! Try to get lower.");
            } else {
                println!("Your guess is too far! Try to get higher.");
            }
            tries -= 1;
        } else if 20 <= diff_abs && diff_abs < 30 {
            if diff > 0 {
                println!("Your guess is a bit too far! Try to get lower.");
            } else {
                println!("Your guess is a bit too far! Try to get higher.");
            }
            tries -= 1;
        } else if 10 <= diff_abs && diff_abs < 20 {
            if diff > 0 {
                println!("Your guess is not too far! Try to get lower.");
            } else {
                println!("Your guess is not too far! Try to get higher.");
            }
            tries -= 1;
        } else if 0 < diff_abs && diff_abs < 10 {
            if diff > 0 {
                println!("Your guess is really close to my number! Try to get lower.");
            } else {
                println!("Your guess is really close to my number! Try to get higher.");
            }
            tries -= 1;
        } else {
            println!("Congratulations! You guessed the number: {random_number}");
            println!("=======================================================================================");
            println!("The Game is over. Thanks for the playing!");
            println!("The number was: {random_number}");
            println!("Bye!");

            break;
        }

        if tries == 0 && !second_chance {
            println!("=======================================================================================");
            println!("It looks like you are out of tries! The game can be over or ...");
            println!("You want a second chance maybe? But you'll ony have 5 tries for now.");
            print!("Write 'yes' or 'no' for your choice: ");
            match io::stdout().flush() {
                Ok(_) => {},
                Err(_) => {
                    println!("=======================================================================================");
                    println!("Sorry, failed to flush the output! :(");
                    println!("Restaring the Game from the last random value I thought of (10 tries restored)...");
                    start_greeting();

                    second_chance = false;
                    tries = 10;
                    continue;
                }
            };

            let mut user_choice = String::new();
            match io::stdin().read_line(&mut user_choice) {
                Ok(_) => {},
                Err(_) => {
                    println!("=======================================================================================");
                    println!("Sorry, failed to read the line of your thought! :(");
                    println!("Restaring the Game from the last random value I thought of (10 tries restored)...");
                    start_greeting();

                    second_chance = false;
                    tries = 10;
                    continue;
                }
            };

            if user_choice.trim().to_lowercase() == "yes" {
                println!("Great! Let's start from the begining!");
                second_chance = true;
                tries = 5;
            } else if user_choice.trim().to_lowercase() == "no" {
                println!("Ok! So, the real random number was: {random_number}");
                println!("You can always try again later if you want to.");
                println!("Bye!");

                break;
            } else {

                /*
                0 - program's failure
                1 - user choice is 'yes'
                2 - user choice is 'no'
                */

                let wants_to_continue : u8 = continue_or_not();

                if wants_to_continue == 1 {
                    println!("Great! Let's start from the begining!");
                    second_chance = true;
                    tries = 5;
                } else if wants_to_continue == 2 {
                    println!("Ok! So, the real random number was: {random_number}");
                    println!("You can always try again later if you want to.");
                    println!("Bye!");

                    break;
                } else {
                    println!("=======================================================================================");
                    println!("Sorry, something went wrong with your choice! :(");
                    println!("Restaring the Game from the last random value I thought of (10 tries restored)...");
                    start_greeting();

                    second_chance = false;
                    tries = 10;
                    continue;
                }
            }

        } 
        
        if tries == 0 && second_chance {
            println!("=======================================================================================");
            println!("Sorry, but you're out of your second chance tries! :(");
            println!("So, the real random number was: {random_number}");
            println!("You can always try again later if you want to.");
            println!("Bye!");

            break;
        }

    }

}

fn start_greeting() {
    println!("=======================================================================================");
    println!("Welcome to the Randomizer Game!");
    println!("The rules are simple: I thought of a number between 0 and 100 and you have to guess it.");
    println!("You only have 10 tries. Ready? Let's go!");
}

fn check_float_guess(guess : f64) -> f64 {
    if guess - guess.trunc() != 0.0 {
        println!("Actually, I expected an integer, but you gave me a float.");
        println!("It will be rounded to the nearest integer: {guess} -> {}", guess.round());

        return guess.round();
    }
    
    return guess;
}

fn check_negative_guess(guess : f64) -> f64 {
    if guess < 0.0 {
        println!("Actually, it was said about the range of 0 to 100, but you gave me a negative number.");
        println!("It will be understood as positive: {guess} -> {}", guess.abs());

        return guess.abs();
    }

    return guess;
}

fn continue_or_not() -> u8 {

    println!("Sorry, I expected 'yes' or 'no', but you gave me something else.");
    print!("Please, write 'yes' or 'no' for your choice: ");
    match io::stdout().flush() {
        Ok(_) => {},
        Err(_) => {
            return 0;
        }
    };

    loop {

        let mut user_choice = String::new();
        match std::io::stdin().read_line(&mut user_choice) {
            Ok(_) => {},
            Err(_) => {
                return 0;
            }
        }

        if user_choice.trim().to_lowercase() == "yes" {
            return 1;
        } else if user_choice.trim().to_lowercase() == "no" {
            return 2;
        } else {
            println!("Sorry, I expected 'yes' or 'no', but you gave me something else.");
            print!("Please, write 'yes' or 'no' for your choice: ");
            match io::stdout().flush() {
                Ok(_) => {},
                Err(_) => {
                    return 0;
                }
            };
        }
    }
}