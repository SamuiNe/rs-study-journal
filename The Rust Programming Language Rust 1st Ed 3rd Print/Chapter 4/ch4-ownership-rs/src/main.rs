fn scope_test(){            //s is not valid here; it is not yet declared
    // Note - This one is immutable string.
    let s = "hello";   //s is valid from this point forward

    println!("{}", s);      //do stuff with s
}                           //this scope is now over, and s is no longer valid

fn string_from_heap(){
    //This one is mutable, but literals is not.
    //Note - This is due to how String literals are handled by the compiler.
    //       For String literals, the text is hardcoded directly to the final executable.
    //       For this String data type, it is allocated to the heap requested by the OS at runtime.
    //Note - For Rust, using the drop function is not required as it is automatically freed by
    //       Garbage Collector when it is out of scope.
    let mut s = String::from("hello"); //s is valid from this point forward
    //do stuff with s
}                                               //this scope is now over, and s is no longer valid

fn ways_variable_and_data_interact_move(){
    //For this case, these two does the following:
    //          - Bind the value 5 to x
    //          - Make a copy of the value in x and bind it to y
    //Note - Both x and y are stored in stack.
    let x = 5;
    let y = x;

    //For this one however, these two does the following:
    //          - String::from("Hello") creates an array of char of index 0 to 4,
    //              ["h", "e", "l", "l", "o"].
    //          - In which, it is allocated on the heap and requested by the operating system in
    //              runtime.
    //          - s1 would then have a ptr pointed towards the 0 index of the array with len of 5
    //              and capacity of 5.
    //          - s2 would then copy the ptr, len, and capacity from s1.
    //          - In which, it would result s2 pointing to the same array of char as s1.
    //          - However, Rust also considers s1 invalid as it is already moved to s2. Hence,
    //              referencing s1 after the s2 = s1 would cause an error due to borrow of moved
    //              value. Rust does this to prevent a situation called "double free", in which two
    //              variables are trying to free the same memory. This can cause memory corruption.
    //Note - Rust's move is a shallow copy, and Rust will never automatically deep copy a heap data.
    let s1 = String::from("Hello");
    let s2 = s1;

    /*
    println!("{}, world!", s1); // This one throws a "Borrow of moved value" error.
     */
}

fn ways_variable_and_data_interact_clone(){
    //clone needs to be called if one needs a deep copy of a data.
    //In this case, the heap data does get copied.
    let s1 = String::from("hello");
    let s2 = s1.clone();

}

fn stack_only_data_copy(){
    //However for this case, x is still can be referenced after it is assigned to variable y.
    //Rust does this because in this case x is stored in a stack, and it is fast to copy x value
    //to y.
    //Note - Calling clone here does not do anything different, so it is optional.
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x , y);

    //Notes - Rust have a special annotation called "copy" trait to allow deep copy, but under a
    //        condition where the type is not annotated and theres is no "drop" trait implemented.
    //Notes - In addition, the following types can have a "copy" (Not all, but some):
    //          All integer types, boolean types, character type (char), floats,
    //          and tuples (if all types within the tuple are copyable)
}

fn ownership_and_functions(){
    let s = String::from("hello");  //s comes into scope

    takes_ownership(s);            //s's value moves into the function,
                                             //and so is no longer valid here

    let x = 5;                          //x comes into scope

    makes_copy(x);               //x would move into the function, but i32 is Copy,
                                            //so it's okay to still use x afterward

    //Warning - If we use s after the call to takes_ownership, compile time error will occur.
}

fn takes_ownership(some_string: String) {   //some_string comes into scope
    println!("{}", some_string);
} //Here, some_string goes out of scope and 'drop' is called. The backing memory is freed.

fn makes_copy(some_integer: i32){           //some_integer comes into scope
    println!("{}", some_integer);
} //Here, some_integer goes out of scope. Nothing special happens.

fn return_values_and_scope(){
    let s1 = gives_ownership();          //gives_ownership moves its return value into s1

    let s2 = String::from("hello");   //s2 comes into scope

    let s3 = takes_and_gives_back(s2);         //s2 is moved into takes_and_gives_back,
                                               //which also moves its return value into s3

}   //Here, s3 goes out of scope and is dropped. s2 goes out of scope but was moved,
    //so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {                        //gives_ownership will move its return value
                                                        //into the function that calls it

    let some_string = String::from("hello");   //some_string comes into scope

    some_string                                         //some_string is returned and moves out to
                                                        //the calling function
}

//takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String{ //a_string comes into scope

    a_string // a_string is returned and moves out to the calling function
}

fn return_values_and_scope_part_b(){
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

}

fn calculate_length(s: String) -> (String, usize){
    let length = s.len(); //len() returns the length of a String

    (s, length)
}

fn main() {
    scope_test();
    string_from_heap();
    ways_variable_and_data_interact_move();
    ways_variable_and_data_interact_clone();
    stack_only_data_copy();
    ownership_and_functions();
    return_values_and_scope();
    return_values_and_scope_part_b();
}
