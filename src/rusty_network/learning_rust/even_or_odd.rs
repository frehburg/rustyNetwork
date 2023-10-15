fn even_or_odd(i: i32) -> bool {
    if i % 2 == 0 { true } else { false }
}

pub fn test_even_or_odd() {
    println!("Is 1 even? {}", even_or_odd(1));
    println!("Is 2 even? {}", even_or_odd(2));
    println!("Is -69 even? {}", even_or_odd(-69));
    println!("Is 0 even? {}", even_or_odd(0));
    println!("Is 420 even? {}", even_or_odd(420));
}