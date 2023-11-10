// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message

use std::i8;

#[derive(Debug)]
struct Adult {
    name: String,
    age: i8,
}

impl Adult {
    fn new(name: String, age: i8) -> Result<Self, String> {
        if age >= 21 {
            return Ok(Self { name, age });
        } else {
            return Err("An adult must be 21 or older.".to_owned());
        }
    }
}

fn print(result: Result<Adult, String>) {
    match result {
        Ok(adult) => println!("{:?}", adult),
        Err(e) => println!("{:?}", e)
    };
}

fn main() {
    let bob = Adult::new(String::from("Bob"), 18);
    let alice = Adult::new("Alice".to_owned(), 25);
    
    print(bob);
    print(alice);
}
