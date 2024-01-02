// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavour {
    Sweet,
    Sparkling,
    Juicy,
    Sour,
}

struct Drink {
    flavour: Flavour,
    ounces: f64,
}

fn print_drink(drink: Drink) {
    match drink.flavour {
        Flavour::Sweet => println!("sweet"),
        Flavour::Sparkling => println!("sparkling"),
        Flavour::Juicy => println!("juicy"),
        Flavour::Sour => println!("sour"),
    }
    println!("{:?}", drink.ounces);
}

fn main() {
    let d = Drink {
        flavour: Flavour::Sweet,
        ounces: 3.15,
    };

    print_drink(d);
}
