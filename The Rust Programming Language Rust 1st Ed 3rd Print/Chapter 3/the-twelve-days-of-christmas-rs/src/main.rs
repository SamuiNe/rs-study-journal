use std::io;

fn ordinal_numbers_output(verse_number : usize){
    let ordinal_numbers : [&str; 13] = ["", "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    println!("On the {} day of Christmas, my true love sent to me",
             ordinal_numbers[verse_number]);
}

fn verse_output(verse_number : usize){
    //Based on Twelve Days of Christmas provided by Genius
    //Source: https://genius.com/Christmas-songs-the-twelve-days-of-christmas-lyrics
    let song_lyrics : [&str; 13] = ["", "A partridge in a pear tree", "Two turtle doves, and",
        "Three french hens", "Four calling birds", "Five golden rings", "Six geese a-laying",
        "Seven swans a-swimming", "Eight maids a-milking", "Nine ladies dancing",
        "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"];

    let mut counter: usize = verse_number;

    while counter > 0 {
        println!("{}", song_lyrics[counter]);

        counter -= 1;
    }
}

fn main() {
    println!("Welcome to The Twelve Days of Christmas Program!");
    println!("Please enter the verse you want to choose for the lyrics [Verse 1 - 12].");

    let mut verse_number : String = String::new();
    io::stdin().read_line(&mut verse_number)
        .expect("Failed to read line, please try again!");
    let verse_number: usize = verse_number.trim().parse()
        .expect("Not a valid number, please try again!");

    if verse_number > 0 && verse_number <= 12{
        ordinal_numbers_output(verse_number);
        verse_output(verse_number);
    }
    else {
        println!("Invalid verse number, please enter number between 1 and 12!")
    }
}
