use std::io;

mod exercises;

fn main() {
    println!("Choose an exercise to run:");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    match choice.trim() {
        "printing" => {
            println!("Choose an printing exercise (hello_world/sum_numbers/greet/print_vec):");
            let mut intro_choice = String::new();
            io::stdin().read_line(&mut intro_choice).expect("Failed to read input");

            match intro_choice.trim() {
                "hello_world" => exercises::printing::run_hello_world(),
                "sum_numbers" => exercises::printing::run_sum_numbers(),
                "greet" => {
                    let mut name = String::new();
                    println!("Enter your name:");
                    io::stdin().read_line(&mut name).expect("Failed to read input");
                    exercises::printing::run_greet(name.trim());
                }
                "print_vec" => {
                    let mut input = String::new(); // String to store user input
                    println!("Enter a vector of integers separated by spaces:");
                    io::stdin().read_line(&mut input).expect("Failed to read input"); // Split the input string by spaces and convert each element to an integer
                    let vector: Vec<i32> = input
                        .trim() // Remove any extra whitespace
                        .split_whitespace() // Split the string by spaces
                        .filter_map(|s| s.parse().ok()) // Try to parse each part as an integer
                        .collect(); // Collect the results into a Vec<i32>
                    exercises::printing::run_print_struct(vector);
                }
                _ => println!("Unknown printing exercise!"),
            }
        }
        "second" => { println!("Not implemented yet") }
        _ => println!("Unknown exercise!"),
    }
}
