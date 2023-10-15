pub struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: &str, age: u32) -> Person {
        Person {
            name: String::from(name),
            age,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_age(&self) -> u32 {
        self.age
    }

    pub fn say_hello(&self) {
        println!("Hello, my name is {} and I'm {} years old.", self.name, self.age);
    }

    pub fn have_birthday(&mut self) {
        self.age += 1;
        println!("Happy Birthday, {}! You're now {} years old.", self.name, self.age);
    }
}
