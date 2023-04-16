use std::io::{stdin};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    
    let secret_number = rand::thread_rng().gen_range(1..100);

    println!("Welcome to number guessing game!");
    println!("Powred by Rust Programming Language\n");

    loop {
        let mut input = String::new();
        
        println!("Enter command (help, start, quit): ");
        
        stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
        let input: &str = input.trim();
        println!("");

        match input {
            "start" => {
                number_guessing_game(&secret_number);
            },
            "s" => {
                number_guessing_game(&secret_number);
            },
            "help" => {
                help()
            },
            "h" => {
                help()
            },
            "quit" =>{
                return;
            },
            "q" => {
                return;
            },
            _ => println!("You have entered an invalid command!")
        };
    }

    
}


fn help(){
    println!("\nWelcome to NGG\n");
    println!("help : Lists out all the command that can be used.");
    println!("start : Starts the Number Guessing Game");
    println!("quit, q : Quits the program");
}

fn number_guessing_game(secret_number: &u32){
    loop {
        let mut guess:String = String::new();
        println!("Enter your guess: ");
        stdin().read_line(&mut guess).expect("Failed to read line");
        
        let guess: &str = guess.trim();
        match guess {
            "q" => {
                break
            },
            "quit" => {
                break
            },
            _ => {
                let guess:u32 = match guess.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("You have entered a invalid number!");
                        continue
                    }
                };
        
                match guess.cmp(secret_number) {
                    Ordering::Less => println!("Too small!"),
                    Ordering::Greater => println!("Too big!"),
                    Ordering::Equal => {
                        println!("Congratulations!");
                        break;
                    },
                }
            }
        }
    };
    println!("");
}
