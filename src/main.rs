use std::io;

fn main() {
    println!("Enter a number:");

    let mut number_input = String::new();

    io::stdin()
        .read_line(&mut number_input)
        .expect("Something went wrong.");

    let number_input: i32 = match number_input.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Please enter a number.");
            0
        }
    };

    for i in 0..number_input {
        println!("fib{}: {}", i, fib(i));
    }
}

fn fib(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
} 