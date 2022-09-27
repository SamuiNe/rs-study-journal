use std::io;

fn variables_and_mutability(){
    //Note - In Rust, variables are immutable by default.
    //       In order to make it mutable, one must use the keyword 'mut' after 'let'.
    let mut x = 5;

    //Note - Except when the variable have the keyword 'const'.
    //       'mut' keyword is not possible since 'const' makes it always immutable.
    const MAX_POINTS: u32 = 100_000;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //Note - This part compiles and calculates with no errors due to the concept of "Shadowing".
    //       In which, the second instance or later of same named variable is shadowing the
    //          previous value of the same named variable.
    //       In this case:
    //          x = 5 (1st instance)
    //          x = x (1st instance value = 5) + 1
    //       which results in...
    //          5 + 1 = 6
    //       and assigned to the second instance of x.
    //       Same thing for third instance, in which it would be:
    //          x = 6 (2nd instance)
    //          x = x (2nd instance value = 6) * 2
    //       which results in...
    //          6 * 2 = 12
    //       and assigned to the third instance of x.
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    //Note - In which, the third instance of x is used for the println- hence it outputs 12.
    println!("The value of x is: {}", x);

    //Note - This one is ok too, spaces will end up storing the length of first instance of space.
    let spaces = "   ";
    let spaces = spaces.len();

    //Note - But this one is not fine. Since the first instance expects a data type of &str.
    /*
        let mut spaces = "   ";
        spaces = spaces.len();
     */
}

fn scalar_types() {
    //Note - This one throws "type annotations needed"  error since Rust needs to know
    //          the data type for all variables.
    //       Rust compiler does not know which type to use since there are many possible types
    //          for this line of code.
    /*
        let guess = "42".parse().expect("Not a number!");
     */
    //Note - In which, one can correct it by adding a type.
    //       In this case, its : u32 for unsigned 32bit data type (or any integer or float is ok).
    //Warning - Compiling in debug mode includes checks for Integer overflow.
    //          If Integer overflow occurs, the program produces unrecoverable error named 'panic'.
    //          This check is not included if one compiles the program with --release flag.
    let guess : u32 = "42".parse().expect("Not a number!");

    //Note - By default, Rust initializes floats as 64bit floats.
    //       Hence, if one wants to use floats other than 64bit floats, they need to explicitly
    //          write it during the initialization process.
    let x = 2.0; // This one is float 64bit data type.
    let y: f32 = 3.0; // This one is float 32bit data type.

    //addition
    //This results in i32 data type.
    let sum = 5 + 10;

    //subtraction
    //This results in f64 data type.
    let difference = 95.5 - 4.3;

    //multiplication
    //This results in i32 data type.
    let product = 4 * 30;

    //division
    //This results in f64 data type.
    let quotient = 56.7 / 32.2;

    //remainder
    //This results in i32 data type.
    //Note - One cannot use 43 mod 5. 'mod' is a reserved keyword for modules.
    let remainder = 43 % 5;

    //Rust boolean data type (implicit and explicit form)
    let t = true;
    let f: bool = false;

    //Rust character type
    //Note - Rust char type supports the following characters:
    //          ASCII, accented letters, Chinese, Japanese, Korean, emojis (oh my), and zero-width.
    let c = 'z';
    let z = 'ƶ';
    //Book shows heart_eyed_cat char variable with heart_eyed_cat emoji, I am not typing this.
}

fn compound_types() {
    //Rust tuples can structure multiple data types in a single tuple. Pretty neat to be honest.
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    //In which it can be destructured using something like below. Actually pretty neat.
    let (x, y, z) = tup;

    //In which, the output for this line is actually 6.4 (in f64 data type).
    println!("The value of y is: {}", y);

    //The following also works, index starts at 0 (unlike Lua).
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    //Rust Array data type, this one does not accept multiple data types.
    //Note - Like C++ Arrays, Rust Arrays cannot be grow or shrink after the fact.
    //Array of 5 i32s
    let a = [1, 2, 3, 4, 5];

    //Array of 12 &str
    let months = ["January", "February", "March", "April", "May", "June", "July", "August",
        "September", "October", "November", "December"];

    //To explicitly indicate data type on Rust Array, one must do the following:
    //      let <variable_name>: [<data_type>; <number_of_elements>] = [<array_of_data_type>]
    //Note - Number of elements at the left hand side must match the ones at the right hand side.
    //Note - Number of elements at the left hand side starts with 1, not 0.
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    //Basically a way of initializing array of 3s repeated 5 times ([3, 3, 3, 3, 3]).
    let a = [3; 5];

    //Initializing and accessing the first element of an array.
    //Note - a.0 does not work since a.0 is tuple indexing. Array indexing is a[0].
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
}

//Enter anything greater than 4 for index out of bounds error
fn invalid_array_element_access_demo(){
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line, please try again!");
    let index: usize = index.trim().parse().expect("Not a number");

    let element = a[index];

    println!("The value of the element at index {} is: {}", index, element);
}

// Simple Hello World with 2 functions for Rust
// Note - One does not need to keep referenced functions above the function call line.
//      As long it is defined somewhere it is ok for Rust!
fn function_demo1(){
    println!("Hello world!");

    another_function1();
}

fn another_function1(){
    println!("Another function!");
}

// Passing value of 11 to another_function2's x parameter
fn function_demo2(){
    another_function2(11);
}

//Note - Rust requires type annotations on every parameter of a function, if there is at least one.
fn another_function2(x: i32){
    println!("The value of x is: {}", x);
}

//Passing value of 22 and 33 to another_function3's x and y parameter, respectively
fn function_demo3(){
    another_function3(22, 33);
}

fn another_function3(x: i32, y: i32){
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

//Note - The printed x is still 5 since the x inside the y variable initialization is out of scope.
//Note - The printed y is 4 since the expression inside y is x + 1 (In which, the inner x is 3).
//Note - The 'let' keyword itself does not return any value.
//Note - Rust does not allow x = y = 3 initialization.
fn scopes_test(){
    let x = 5;

    let y = {
        let x = 3;
        println!("The value of inner x is: {}", x);
        x + 1
    };

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

//This function returns the value 5 with data type of i32.
//Note - To declare a data type for the function return value, the format is the following:
//              function() -> return_value_data_type
fn five() -> i32{
    5
}

//Note - If semicolon is added at the end of this line, it would throw mismatched types () in i32.
//       x + 1 is the correct syntax for this case.
fn plus_one(x: i32) -> i32 {
    x + 1
}

//In which, the x value should print 5.
//Note - One can also do five() instead of x. But is it less or more readable?
fn checking_with_function_returning_value(){
    let x = five();

    println!("The value of x is: {}", x);
    println!("The value of five is: {}", five());

    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn rust_comments_tutorial(){
    //Rust Comments Tutorial
    // Hello World!

    // So we're doing something complicated here, long enough that we need
    // multiple lines of comments to do it! Whew! Hopefully, this comment will
    // explain what's going on

    let lucky_number = 7; // I'm feeling lucky today

    //This one is the more common commenting format than the one above.
    //I'm feeling lucky today
    let lucky_number = 7;
}

fn control_flow(){
    let number = 3;

    //This outputs 'condition was true'
    //Note - Block of code associated with the conditions in if expressions is sometimes
    //       called 'arms'.
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 7;

    //This outputs 'condition was false'
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    //Note - This one gives a mismatched types error because the if statement expects a bool.
    //       In which, it additionally outputs "expected type 'bool' found type '{integer}'"
    let number = 3;
    /*
    if number {
        println!("number was three");
    }
    */

    //In which, you can fix it by doing something like this:
    //Output for this one is "number was something other than zero"
    if number != 0 { //This evaluates the variable 'number' and returns true or false (bool)
        println!("number was something other than zero");
    }

    //Handling Multiple Conditions with else if
    let number = 6;

    //Output for this one is 'number is divisible by 3'
    //Note - Be careful to not have too many else ifs. This will clutter your code.
    //       In which, refactoring code may be necessary.
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //Using if in a let Statement
    //Output for this one is 'The value for this number is: 5'.
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of this number is: {}", number);

    // Note - This one throws an error since the if statement takes the first expected value, and
    //        it would then expects the same variable type for second and later values.
    //        In short, if and else statement must evaluate to the same data type.
    let condition = true;

    /*
    let number = if condition {
        5
    } else {
        "six"
    };
     */

    println!("The value of number is: {}", number);
}

fn repetition_with_loops(){
    //Note - This loop indefinitely loops until the user uses the keyboard shortcut CTRL+C if the
    //       'break' keyword is commented.
    loop {
        println!("again!");
        break;
    }

    //Note - This loop increments the counter until the value 10, in which it would then return 20
    //       because of break counter(value is 10 by the last iteration) * 2;
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10{
            break counter * 2;
        }
    };

    //Loop continues until the number reaches 0 (More accurately loop continues when the value
    //is non-zero.
    //For this one, it outputs "3!", "2!", "1!", and "LIFTOFF!!!"
    let mut number = 3;

    while number != 0{
        println!("{}!", number );

        number = number - 1;
    }

    println!("LIFTOFF!!!");

    //The loop keeps looping until it reaches the end of the array, which is index 4.
    //For this one, it outputs 10, 20, 30, 40, and 50 in that order.
    //Warning - The index value must be changed when the array size is changed.
    //          It may cause out of bounds error or logical error!
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5{
        println!("the value is: {}", a[index]);

        index = index + 1;
    }

    //For loop version of the while loop
    //This is the most commonly used loop for Rust users.
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    //Similar loop as the liftoff one, but this one is using range (1..4 thing) and reverse (rev)
    //Looks pretty cool to be honest!
    for number in (1..4).rev(){
        println!("{}!", number);
    }

    println!("LIFTOFF!");
}

fn main(){
    variables_and_mutability();
    scalar_types();
    compound_types();
    invalid_array_element_access_demo();
    function_demo1();
    function_demo2();
    function_demo3();
    scopes_test();
    checking_with_function_returning_value();
    rust_comments_tutorial();
    control_flow();
    repetition_with_loops();
}