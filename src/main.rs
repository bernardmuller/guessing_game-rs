use std::io;
use rand::Rng;
use std::cmp::Ordering;

const MAX_GUESS_NUMBER: i32 = 5;

fn check_game_end(guess: i32) -> bool {
    if guess == MAX_GUESS_NUMBER {
        println!("--------------------------");
        println!("Game over ☹️");
        return true;
    };
    false
}

fn main() {
    println!("Guess a number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut guess_number = 0;

    loop {

        let game_over = check_game_end(guess_number);
        if game_over {break};

        println!("--------------------------");
        println!("Guesses Left: {}", MAX_GUESS_NUMBER - guess_number);
        println!("Please input your guess:");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        guess_number += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small. 🦔"),
            Ordering::Greater => println!("Too Big. 🐘"),
            Ordering::Equal => {
            println!("--------------------------");
                println!("You win!! 😃");
                break;
            },
        };
        
        
   }
}
