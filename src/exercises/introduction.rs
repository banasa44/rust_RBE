pub fn run_hello_world() {
    println!("Hello, World!");
}

pub fn run_sum_numbers() {
    let a = 5;
    let b = 3;
    println!("The sum of {} and {} is {}", a, b, a + b);
}

pub fn run_greet(name: &str) {
    println!("Hello, {}!", name);
}
