#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod rusty_network;

use rusty_network::learning_rust::person::Person;
use rusty_network::learning_rust::even_or_odd;
use rusty_network::learning_rust::factorial;

fn main() {
    factorial::test_factorial();
}

fn test_person() {
    let mut person = Person::new("Filip", 21);

    println!("Name: {}", person.get_name());
    println!("Age: {}", person.get_age());

    person.say_hello();
    person.have_birthday();
    println!("New age: {}", person.get_age());
}
