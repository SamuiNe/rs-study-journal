use std::io;

fn fibonacci_number_list(index : usize) -> i32{
    //First 20 fibonacci number (Including the 0 index one)
    let fibonacci_number_list : [i32; 21] = [0, 1, 1, 2, 3, 5,
        8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765];
    return fibonacci_number_list[index];
}

fn main() {
    //TODO: Generating nth fibonacci numbers instead of hard coding, and using array would be more
    //      elegant and flexible, no?
    //      Be sure to keep in mind of i32 limitations though, maybe change to u32 instead of i32
    //      while at it?
    println!("Welcome to nth Fibonacci Number program!");
    println!("Note - Only index in range of 0 to 20 inclusive is supported.");
    println!("Please choose the index you want for the Fibonacci number calculation.");

    let mut fibonacci_index = String::new();
    io::stdin().read_line(&mut fibonacci_index)
        .expect("Failed to read line, please try again!");
    let fibonacci_index: usize = fibonacci_index.trim().parse()
        .expect("Not a number, please try again!");

    //No need to do fibonacci_index >= 0 due to type limit (usize cannot be less than 0)
    if fibonacci_index <= 20{
        let results = fibonacci_number_list(fibonacci_index);
        println!("The Fibonacci number for index {} is {}!", fibonacci_index, results);
    }
    else {
        println!("Invalid index, please enter index between 0 and 20 inclusive!")
    }
}
