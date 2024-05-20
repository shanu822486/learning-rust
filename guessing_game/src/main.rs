use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop{
        println!("Please guess the number");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).
        expect("Error while entering the input");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("failed to parse the int");
                continue;
            }
        };

        match guess.cmp(&secret_number){
            Ordering::Less => println!("guessed number is smaller"),
            Ordering::Greater => println!("guessed number is greater"),
            Ordering::Equal => {
                println!("Guessed number is Equal");
                break;
            }
        };
    }
}
