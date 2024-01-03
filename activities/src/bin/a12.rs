// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Colour {
    Green,
    Blue,
    Red,
}

impl Colour {
    fn print(&self) {
        match self {
            Colour::Green => println!("colour: green"),
            Colour::Blue => println!("colour: blue"),
            Colour::Red => println!("colour: red"),
        }
    }
}

struct ShippingBox {
    dimensions: (i32, i32, i32),
    weight: f64,
    colour: Colour,
}

impl ShippingBox {
    fn new(dimensions: (i32, i32, i32), weight: f64, colour: Colour) -> Self {
        ShippingBox {
            dimensions,
            weight,
            colour,
        }
    }

    fn print_characteristics(&self) {
        println!(
            "dimensions: {:?}x{:?}x{:?}",
            self.dimensions.0, self.dimensions.1, self.dimensions.2
        );
        println!("weight: {:?}", self.weight);
        self.colour.print();
    }
}
fn main() {
    let blue_box = ShippingBox::new((1, 2, 3), 45.4, Colour::Blue);
    blue_box.print_characteristics();

    let red_box = ShippingBox::new((4, 5, 6), 12.1, Colour::Red);
    red_box.print_characteristics();

    let green_box = ShippingBox::new((7, 8, 9), 65.3, Colour::Green);
    green_box.print_characteristics();
}
