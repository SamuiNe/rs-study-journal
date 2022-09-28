use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to 'Guess the Number'!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop{
        println!("Please input your guess below: ");

        let mut guess = String::new();

        //read_line returns io::Result, in which it have 2 variants be 'Ok' or 'Err'.
        //.expect gets triggered when the variant 'Err' of io::Result is returned by read_line.
        //Warning - triggering .expect also crashes the program.
        //Note - If theres no .expect, a warning will be thrown out during compilation.
        //       In this case, proper error handling must be done.
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line, please try again!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Guessed number is too small!"),
            Ordering::Greater => println!("Guessed number is too large!"),
            Ordering::Equal => {
                println!("You guessed the correct number! Congratulations!");
                break;
            }
        }
    }
}
