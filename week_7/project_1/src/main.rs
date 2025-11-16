use std::io;

// Function to read a float input from the user
fn read_input(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a number."),
        }
    }
}

// Area of Trapezium: height/2 * (base1 + base2)
fn area_trapezium() {
    let height = read_input("Enter height:");
    let base1 = read_input("Enter base 1:");
    let base2 = read_input("Enter base 2:");
    let area = (height / 2.0) * (base1 + base2);
    println!("The Area of the Trapezium is: {:.2}\n", area);
}

// Area of Rhombus: 1/2 * diagonal1 * diagonal2
fn area_rhombus() {
    let d1 = read_input("Enter diagonal 1:");
    let d2 = read_input("Enter diagonal 2:");
    let area = 0.5 * d1 * d2;
    println!("The Area of the Rhombus is: {:.2}\n", area);
}

// Area of Parallelogram: base * altitude
fn area_parallelogram() {
    let base = read_input("Enter base:");
    let altitude = read_input("Enter altitude:");
    let area = base * altitude;
    println!("The Area of the Parallelogram is: {:.2}\n", area);
}

// Area of Cube: 6 * (length of the side)^2
fn area_cube() {
    let side = read_input("Enter side length:");
    let area = 6.0 * side.powi(2);
    println!("The Surface Area of the Cube is: {:.2}\n", area);
}

// Volume of Cylinder: pi * radius^2 * height
fn volume_cylinder() {
    let radius = read_input("Enter radius:");
    let height = read_input("Enter height:");
    let volume = std::f64::consts::PI * radius.powi(2) * height;
    println!("The Volume of the Cylinder is: {:.2}\n", volume);
}


fn main() {
    println!("*** MTH 101 Shape Calculator ***\n");

    loop {
        // 1. Prompt user to select an equation
        println!("Select a calculation:");
        println!("1. Area of Trapezium");
        println!("2. Area of Rhombus");
        println!("3. Area of Parallelogram");
        println!("4. Surface Area of Cube");
        println!("5. Volume of Cylinder");
        println!("6. Exit");

        let choice = read_input("Enter your choice (1-6):");

        // 2. Perform the corresponding calculation
        match choice as u32 {
            1 => area_trapezium(),
            2 => area_rhombus(),
            3 => area_parallelogram(),
            4 => area_cube(),
            5 => volume_cylinder(),
            6 => {
                println!("Exiting program. Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please select a number from 1 to 6.\n"),
        }
    }
}
