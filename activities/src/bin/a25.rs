// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Perimeter {
    fn calculate_perimeter(&self) -> f32;
}

struct Square {
    side: f32,
}

struct Triangle {
    side_a: f32,
    side_b: f32,
    side_c: f32,
}

impl Perimeter for Square {
    fn calculate_perimeter(&self) -> f32 {
        self.side * 4.0
    }
}

impl Perimeter for Triangle {
    fn calculate_perimeter(&self) -> f32 {
        self.side_a + self.side_b + self.side_c
    }
}

fn print_perimeter(shape: impl Perimeter) {
    println!("Perimeter: {:?}", shape.calculate_perimeter());
}

fn main() {
    let square = Square { side: 5.0 };
    let triangle = Triangle {
        side_a: 2.0,
        side_b: 3.0,
        side_c: 4.0,
    };

    print_perimeter(square);
    print_perimeter(triangle);
}
