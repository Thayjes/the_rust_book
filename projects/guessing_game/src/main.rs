use std::{io, cmp::Ordering};
use rand::Rng;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Input your guess!");
        // let user_input: String = read_string();
        // let user_input: u32 = user_input.trim().parse().expect("Please type a number");
        let user_input = read_number();
        if user_input==0 {
            continue;
        }
        println!("You guessed: {user_input}");
    
        match user_input.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }  
        }
    }
}

fn read_string() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("cannot read user input");
    input
}

fn read_number() -> u32 {
    let input = read_string();
    let result: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => return 0,
    };
    result
}