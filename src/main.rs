use std::io;

mod exercises;

fn main() {
    println!("Choose an exercise to run:");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    match choice.trim() {
        "introduction" => {
            println!("Choose an introduction exercise (hello_world/sum_numbers/greet):");
            let mut intro_choice = String::new();
            io::stdin()
                .read_line(&mut intro_choice)
                .expect("Failed to read input");

            match intro_choice.trim() {
                "hello_world" => exercises::introduction::run_hello_world(),
                "sum_numbers" => exercises::introduction::run_sum_numbers(),
                "greet" => {
                    let mut name = String::new();
                    println!("Enter your name:");
                    io::stdin()
                        .read_line(&mut name)
                        .expect("Failed to read input");
                    exercises::introduction::run_greet(name.trim());
                }
                _ => println!("Unknown introduction exercise!"),
            }
        }
        "second" => {
            println!("Not implemented yet")
        }
        _ => println!("Unknown exercise!"),
    }
}
