// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Colour {
    Black,
    Green,
    Blue,
    Red,
    Purple,
}

fn print_colour(colour: Colour) {
    match colour {
        Colour::Black => println!("black"),
        Colour::Green => println!("green"),
        Colour::Blue => println!("blue"),
        Colour::Red => println!("red"),
        Colour::Purple => println!("purple"),
    }
}

fn main() {
    print_colour(Colour::Red);
    print_colour(Colour::Black);
    print_colour(Colour::Green);
    print_colour(Colour::Blue);
    print_colour(Colour::Purple);
}
