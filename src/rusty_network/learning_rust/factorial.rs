fn factorial(i: i32) -> i32 {
    if i == 0 { 1 }
    else { i * factorial(i - 1) }
}

pub fn test_factorial() {
    println!("Factorial of 5 is {} expected 120", factorial(5));
    println!("Factorial of 10 is {} expected 3628800", factorial(10));
    println!("Factorial of 15 is {} expected 1307674368000", factorial(15));
}