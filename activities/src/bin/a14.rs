// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i32,
    name: String,
    fav_color: String,
}

fn print(data: &str) {
    println!("{:?}", data);
}

impl Person {
    fn new(age: i32, name: String, fav_color: String) -> Self {
        Self {
            age,
            name,
            fav_color,
        }
    }

    fn print(&self) {
        print(&self.name);
        print(&self.fav_color);
    }
}

fn main() {
    let people = vec![
        Person::new(5, String::from("A"), String::from("red")),
        Person::new(11, String::from("B"), String::from("green")),
        Person::new(10, String::from("C"), String::from("blue")),
        Person::new(20, String::from("D"), String::from("yellow")),
        Person::new(3, String::from("E"), String::from("orange")),
        Person::new(4, String::from("F"), String::from("black")),
    ];

    for person in people {
        if person.age <= 10 {
            person.print();
        }
    }
}
