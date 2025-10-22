// Rust program that takes the experience and age
// too determine the annual incentive

use std::io;

fn main()
{
    let mut input = String::new();

    println!("\nEnter the employee's Age || Experience (in years): ");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let experience:f32 = input.trim().parse().expect("Not a valid number");

    if experience >= 40.0
    {
        println!("Your incentive is #1_560_000.0");
    }
    else if experience < 40.0 && experience >30.0
    {
        println!("Your incentive is #1_480_000.0");
    }
    else if experience < 28.0
    {
        println!("Your incentive is #1_300_000.0");
    }
    else if experience == 0.0
    {
        println!("Your incentive is #100_000.0");
    }
}
