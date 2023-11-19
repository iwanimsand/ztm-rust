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

trait Shape {
    fn perimeter(&self) -> f32;
}

struct Square {
    a: f32,
}

struct Triangle {
    a: f32,
    b: f32,
    c: f32,
}

impl Shape for Square {
    fn perimeter(&self) -> f32 {
        self.a * 4.0
    }
}

impl Shape for Triangle {
    fn perimeter(&self) -> f32 {
        self.a + self.b + self.c
    }
}

fn calc_perimeter(shape: impl Shape) {
    println!("Perimeter: {:?}", shape.perimeter());
}

fn main() {
    let square = Square { a: 4.0 };
    let triangle = Triangle {
        a: 1.0,
        b: 2.0,
        c: 3.0,
    };

    calc_perimeter(square);
    calc_perimeter(triangle);
}
