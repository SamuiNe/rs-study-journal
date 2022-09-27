use std::io;

fn celsius_to_fahrenheit(celsius : f32) -> f32{
    (celsius * 1.8) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit : f32) -> f32{
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn main() {
    println!("Welcome to temperature conversion program!");
    println!("Please enter if you want to input Celsius or Fahrenheit.");
    println!("Celcius / Celsius -> Convert Celsius to Fahrenheit");
    println!("Fahrenheit -> Convert Fahrenheit to Celsius");

    let mut temperature_selection : String = String::new();
    io::stdin().read_line(&mut temperature_selection)
        .expect("Failed to read line, please try again!");
    temperature_selection = temperature_selection.to_lowercase();
    let temperature_selection: String = temperature_selection.trim().parse()
        .expect("Not a string, please try again!");
    println!("Please enter the temperature value.");

    let mut temperature_value = String::new();
    io::stdin().read_line(&mut temperature_value)
        .expect("Failed to read line, please try again!");
    let temperature_value: f32 = temperature_value.trim().parse()
        .expect("Not a number, please try again!");

    println!("The inputted string is {}", temperature_selection);

    if temperature_selection == "celsius" || temperature_selection == "celcius"{
        let result = celsius_to_fahrenheit(temperature_value);
        println!("The converted value is {} Fahrenheit.", result);

    } else if temperature_selection == "fahrenheit"{
        let result = fahrenheit_to_celsius(temperature_value);
        println!("The converted value is {} Celsius.", result);

    } else {
        println!("Invalid entry, please try again!");

    }
}
