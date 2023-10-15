mod rusty_network;

use rusty_network::learning_rust::person::Person;

fn main() {
    // Use the 'Person' struct from the 'person' module
    let mut person = Person::new("Filip", 21);

    println!("Name: {}", person.get_name());
    println!("Age: {}", person.get_age());

    person.say_hello();
    person.have_birthday();
    println!("New age: {}", person.get_age());
}
