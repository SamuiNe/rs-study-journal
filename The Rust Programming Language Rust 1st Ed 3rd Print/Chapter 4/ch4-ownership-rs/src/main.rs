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

    //Note - This is valid since the calculate_length returns a tuple, in which it matches
    //       the tuple on the left hand side.
    //Warning - The string part must be returned on the calculate_length for this case, since
    //          s1 was moved to the calculate_length method. If this is not done, the string
    //          would be lost when the passed s1 goes out of scope inside the calculate_length.
    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

}

fn calculate_length(s: String) -> (String, usize){
    let length = s.len(); //len() returns the length of a String

    (s, length)
}


fn references_and_borrowing(){
    let s1 = String::from("Hello!");

    let len = calculate_length_reference(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length_reference(s: &String) -> usize {
    s.len()
}   // Here, s goes out of scope. But because it does not have ownership
    // of what it refers to, nothing happens.

//Note - this function with modifying_a_borrowed_value_part_a function throws the following error:
//       cannot borrow immutable borrowed content '*some_string' as mutable
/*
fn modifying_a_borrowed_value_part_a(){
    let s = String::from("Hello!");

    change_part_a(&s);
}

fn change_part_a(some_string: &String){
    some_string.push_str(" World!");

}
*/

//This one is the corrected version of part a.
fn modifying_a_borrowed_value_part_b(){
    let mut s = String::from("hello");

    change_part_b(&mut s);
}

fn change_part_b(some_string: &mut String){
    some_string.push_str(" World!");

}

//Note - This function throws the following error:
//       cannot borrow 's' as mutable more than once at a time
//       Rust does have this restriction to prevent data races at compile time.
/*
fn mutable_references_part_a(){
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}
*/

fn mutable_references_part_b(){
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}

//Note - Mixing immutable and mutable borrow is not allowed for Rust.
//       When at least one immutable reference is used, all future references needs to be immutable.
//       For this function, it throws the following error:
//       cannot borrow 's' as mutable because it is borrowed as immutable
/*
fn mixing_mutable_and_immutable_references(){
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
}
*/

fn dangling_pointer(){
    let reference_to_nothing = dangle_part_b();
}

//Note - This function causes an error to the function above, with the following error:
//       missing lifetime specifier
/*
fn dangle_part_a() -> &String{ // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String

    &s // We return a reference to the string, s
} // Here, s goes out of scope, and is dropped. Its memory goes away
// Danger!
*/

//The solution for this error
fn dangle_part_b() -> String{
    let s = String::from("hello");

    s
}

fn slice_types_part_a() {
    let mut s = String::from("Hello world!");

    let word = first_word(&s); // word will get the value 5
    s.clear(); // This empties the String, making it equal to ""

    // word still have the value 5 here, but there's no more string that we could
    // meaningfully use the value 5 with. word is now totally invalid!
}

//Finds the first word of a string.
//Step 1 : Convert the s String reference as an array of bytes stored in immutable array bytes
//Step 2 : Create an iterator over the array of bytes using the iter()
//Step 3 : Return each element as part of a tuple using enumerate()
//Note - The first element of the tuple returned from enumerate is the index, and the second
//       element is the reference to the element.
//Example - The tuple looks like this (?):
//          ((0, b'h'), (1, b'e'), (2, b'l'), (3, b'l'), (4, b'o'))
//Note - & must be used for the item because it is the reference of the element from the
//       .iter().enumerate().
//Outcome 1 : Once the function encounters a space, return the current index.
//Outcome 2 : If no space is found, return the length of the string.
//Warning - Since the value is separate from the String, there is no guarantee it will be valid
//          in the future.
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn slice_types_part_b(){
    let mut s = String::from("Hello World!");

    let word = first_word(&s); // word will get the value 5
    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that we could meaningfully
    // use the value 5 with. word is now totally invalid!
}

fn string_slices_part_a(){
    let s= String::from("Hello World!");

    //This is Rust's string slice, in which it is a partial reference to the String.
    //Note - .. syntax is called the range syntax,
    //       where it is formatted as [starting_index..ending_index].
    //Note - If one wants to go from the beginning (zero index), starting index can be omitted.
    //       For example, [..5] on the hello immutable variable like the one below.
    //Note - If one wants to include the last byte of the String, ending index can also be omitted.
    //       For example, [6..] on the world immutable variable like the one below.
    let hello = &s[0..5];
    let world = &s[6..11];
}

fn string_slices_equivalent_a(){
    let s = String::from("hello");

    let len = s.len();

    //These two below does the same thing.
    let slice = &s[3..len];
    let slice = &s[3..];
}

fn string_slices_equivalent_b(){
    let s = String::from("hello");

    let len = s.len();

    //These two below also does the same thing.
    let slice = &s[0..len];
    let slice = &s[..];
}

fn first_word_revised(s: &String) -> &str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

/*
fn string_slices_part_b(){
    let mut s = String::from("Hello World");

    let word = first_word_revised(&s);

    //Note - This line will throw an error, in which the error is the following:
    //       cannot borrow 's' as mutable because it is also borrowed as immutable
    s.clear(); //This will throw an error

    println!("the first word is: {}", word);
}
*/

//Note - This one requires string slices.
fn first_word_revised_2(s: &str) -> &str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn string_slices_part_c(){
    let my_string = String::from("Hello World");

    // first_word works on slices of 'String's
    let word = first_word_revised_2(&my_string[..]);

    let my_string_literal = "Hello World";

    // first_word works on slices of string literals
    let word = first_word_revised_2(&my_string_literal[..]);

    // Because string literals *are* string slices already, this works too,
    // without the slice syntax!
    let word = first_word_revised_2(my_string_literal);

}

fn integer_slices(){
    let a = [1, 2, 3, 4, 5];

    //This one creates a slice [2, 3, 4]
    let slice = &a[1..3];
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
    references_and_borrowing();
    //modifying_a_borrowed_value_part_a();
    modifying_a_borrowed_value_part_b();
    //mutable_references_part_a();
    mutable_references_part_b();
    //mixing_mutable_and_immutable_references();
    dangling_pointer();
    slice_types_part_a();
    slice_types_part_b();
    string_slices_part_a();
    //string_slices_part_b();
    string_slices_part_c();
    integer_slices();

}
