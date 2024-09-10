pub fn run_hello_world() {
    println!("Hello, World!");
}

pub fn run_sum_numbers() {
    let a = 5;
    let b = 3;
    let c = a + /* 15 + */ b;
    println!("The sum of {} and {} is {}", a, b, c);
}

pub fn run_greet(name: &str) {
    println!("Hello, {}!", name);
    println!("Pi is roughly {number:.3}", number = 3.141592);
}

use std::fmt; // Import the `fmt` module.

// Define the `List` struct with a Vec<i32>
pub struct List(Vec<i32>);

// Implement the `fmt::Display` trait for `List`
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0; // Get a reference to the inner vector

        write!(f, "[")?; // Write the opening bracket

        // Iterate over the vector with index and value
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?; // Add a comma after the first element
            }
            // Write both the index (count) and value (v)
            write!(f, "{}: {}", count, v)?;
        }

        // Close the bracket
        write!(f, "]")
    }
}

// Function that accepts a vector and prints the formatted `List`
pub fn run_print_struct(vec: Vec<i32>) {
    let list = List(vec); // Wrap the Vec<i32> in the List struct
    println!("{}", list); // This will use the `fmt::Display` implementation
}
