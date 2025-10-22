// Rust program for solving the roots for a given quadratic equation

use std::io;

fn main()
{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the value of a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the value of b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter the value of c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let determinant:f32 = b.powf(2.0) - 4.0 * a * c;
    let root_1:f32 = (-b + determinant.sqrt()) / (2.0 * a);
    let root_2:f32 = (-b - determinant.sqrt()) / (2.0 * a);

    println!("Root of the equation are: {}", root_1);
    println!("{}", root_2);
}