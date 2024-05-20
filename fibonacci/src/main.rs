use std::io;

fn fibonacci_nth_term(n:u64)-> u64{
    if n == 0 {return 0;}
    if n == 1 {return 1;}

    let mut a = 0;
    let mut b = 1;
    let mut result = 0;
    for _ in 2..=n{
        result = a + b;
        a = b;
        b = result;
    }
    return result;
}

fn main() {
    println!("Enter the term you want");
    
    //variable to take input
    let mut n = String::new();
    
    //user input
    io::stdin()
    .read_line(&mut n)
    .expect("error while reading input");

    //convert user input from string to int
    let n : u64 = match n.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("error while converting string to u64");
            return;
        }
    };

    let result = fibonacci_nth_term(n);
    println!("the {n}th term in fibonacci sequence is: {result}");

}
