extern crate phrases;

use phrases::{english, japanese};

fn main() {
    println!("Hello in English: {}", english::hello());
    println!("Goodbye in Japanese: {}", japanese::goodbye());

    // println!("Hello in Japanese: {}", phrases::japanese::greetings::hello());
    // println!("Goodbye in Japanese: {}", phrases::japanese::farewells::goodbye());
}
