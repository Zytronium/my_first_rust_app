use std::io;
use rand::Rng;

fn main() {
    // Input/output & variables
    println!("What is your name?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("\nHello, {}!", input.trim());
    if input.trim().to_lowercase() == "obama" {
        println!("Obama? What is your last name?");
        io::stdin().read_line(&mut input).unwrap();
        println!("Obama's last name is {}", input.trim());
    }

    // ------------------------------------------------- \\
    // Rand number generation
    let secret = rand::thread_rng().gen_range(1..=1234);
    println!("Secret: ***.");
    println!("Oh... You wanted me to actually SHOW the secret? Okay, fine. It's {}", secret);
    println!("Well now it's not a secret anymore, is it?");

    // ------------------------------------------------- \\
    // Pattern matching
    match secret {
        1 => println!("Secret is 1!"),
        2..10 => println!("Secret is one digit"),
        10 | 11 => println!("Secret is 10 or 11"),
        12..100 => println!("Secret is 2 digits"),
        100 => println!("Secret is 100"),
        101..=999 => println!("Secret is over 100"),
        1234 => println!("Secret is 1234; Nice"),
        _ => println!("Secret is over 999"),
    }

    // ------------------------------------------------- \\
    // Using cmp
    //todo: finish this. Am too tired rn
}
