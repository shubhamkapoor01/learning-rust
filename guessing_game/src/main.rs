use std::io;

fn main() {
    println!("Think of an Integer, don't tell me what it is");
    println!("Give me a range for your secret number");

    println!("Enter the lower bound");
    let mut lower_bound_buffer = String::new();
    io::stdin()
        .read_line(&mut lower_bound_buffer)
        .expect("Failed to read lower bound");
    let mut lower_bound: i64 = lower_bound_buffer.trim().parse().expect("Not a number");

    println!("Enter the upper bound");
    let mut upper_bound_buffer = String::new();
    io::stdin()
        .read_line(&mut upper_bound_buffer)
        .expect("Failed to read upper bound");
    let mut upper_bound: i64 = upper_bound_buffer.trim().parse().expect("Not a number");

    println!("Now I will start guessing, just tell me if your number is higher, lower or equal to my guess");

    while lower_bound <= upper_bound {
        let guess: i64 = (lower_bound + upper_bound) / 2;
        println!("Is your number {}?", guess);

        println!("Enter 'higher' if your number is higher, 'lower' if your number is lower, 'equal' if my guess is correct");
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim() {
            "higher" => lower_bound = guess + 1,
            "lower" => upper_bound = guess - 1,
            "equal" => {
                println!("I guessed your number!");
                break;
            }
            _ => println!("Invalid input, try again"),
        }
    }
}
