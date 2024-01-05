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
    age: u8,
    name: String,
    fav_colour: String,
}

fn print(data: &str) {
    println!("{:?}", data);
}

fn main() {
    let people = vec![
        Person {
            age: 25,
            name: "John".to_owned(),
            fav_colour: "red".to_owned(),
        },
        Person {
            age: 10,
            name: "Jimmy".to_owned(),
            fav_colour: "blue".to_owned(),
        },
        Person {
            age: 7,
            name: "Johnny".to_owned(),
            fav_colour: "green".to_owned(),
        },
    ];
    for person in people {
        if person.age <= 10 {
            print(&person.name);
            print(&person.fav_colour);
        }
    }
}
