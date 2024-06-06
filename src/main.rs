use std::io;
 
fn main() {
    let mut fibonacci: Vec<i128> = vec![0, 1, 1];
    
    println!("{:?}", fibonacci);
    println!("Write a number of Fibonacci number you want to generate!");

    let mut input = String::new();
    
    io::stdin() 
        .read_line(&mut input)
        .expect("Failed to read your input!");

    let input: usize = input.trim().parse().expect("Write a number!");

    for _i in 1..input {
        fibonacci.push(1);
        let formula: i128 = fibonacci[fibonacci.len()-2] + fibonacci[fibonacci.len()-3];
        let n = fibonacci.len();
        fibonacci[n-1] = formula;
    }

    let finalresult = fibonacci[input];

    println!("The {input} number of fibonacci is: {finalresult}");
}