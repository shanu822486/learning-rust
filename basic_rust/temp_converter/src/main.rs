use std::io;

fn fahrenheit_to_celsius(temp:f64)-> f64{
    (temp-32.0)*(5.0/9.0)
}

fn main() {
    println!("Convert fahrenheit to celsius");

    let mut temp = String::new(); //variable to store input from user

    //taking input from user
    io::stdin()
    .read_line(&mut temp)
    .expect("error while reading input");

    //converting input from string to f64
    let temp = match temp.trim().parse::<f64>(){
        Ok(num) => num,
        Err(_) => {
            println!("failed to parse the int");
            return;
        }
    };

    //function calling
    let celsius = fahrenheit_to_celsius(temp);
    println!("Converted value to celsius: {celsius}");
}
