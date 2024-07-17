use std::{io, num::Wrapping};
use num_bigint::BigUint;
use num_traits::One;
use std::time::Instant;
 
fn main() {
    println!("Write a number of Fibonacci number you want to generate!");
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line!");

    let n = n.trim().parse().expect("Write a number!");
    
    let now = Instant::now();

    let result = fib(n);

    println!("The {n} number of fibonacci is: {result}");

    let elapsed = now.elapsed();

    println!("Elapsed: {:?}", elapsed);
    
    
    
    
    
    // OLD CODE
    // let mut fibonacci: Vec<i128> = vec![0, 1, 1];
    
    // println!("Write a number of Fibonacci number you want to generate!");

    // let mut input = String::new();
    
    // io::stdin() 
    //     .read_line(&mut input)
    //     .expect("Failed to read your input!");

    // let input: usize = input.trim().parse().expect("Write a number!");

    // for _i in 1..input {
    //     fibonacci.push(1);
    //     let formula: i128 = fibonacci[fibonacci.len()-2] + fibonacci[fibonacci.len()-3];
    //     let n = fibonacci.len();
    //     fibonacci[n-1] = formula;
    // }

    // let finalresult = fibonacci[input];

    // println!("The {input} number of fibonacci is: {finalresult}");
}

fn fib(n: usize) -> BigUint {
    let mut f0 = BigUint::ZERO;
    let mut f1 = BigUint::one();
    for _ in 0..n {
        let f2 = f0 + &f1;
        f0 = f1;
        f1 = f2;
    }
    f0
}