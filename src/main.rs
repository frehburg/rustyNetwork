mod rusty_network;

use rusty_network::learning_rust::person::Person;
use rusty_network::learning_rust::even_or_odd;

fn main() {
    even_or_odd::test_even_or_odd();
}

fn test_person() {
    let mut person = Person::new("Filip", 21);

    println!("Name: {}", person.get_name());
    println!("Age: {}", person.get_age());

    person.say_hello();
    person.have_birthday();
    println!("New age: {}", person.get_age());
}
