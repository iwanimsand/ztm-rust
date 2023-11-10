// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker: Option<i32>,
}

impl Student {
    fn new(name: String, locker: Option<i32>) -> Self {
        return Self { name, locker };
    }

    fn print(&self) {
        match self.locker {
            Some(num) => println!("name: {:?}, locker number: {:?}", self.name, num),
            None => println!("name: {:?}, no locker assigend", self.name),
        }
    }
}

fn main() {
    let mary = Student::new("Mary".to_owned(), Some(3));
    let bob = Student::new("Bob".to_owned(), None);

    mary.print();
    bob.print();
}
