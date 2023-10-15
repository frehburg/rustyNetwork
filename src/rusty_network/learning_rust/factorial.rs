fn factorial(i: i32) -> Result<i32, String> {
    if i < 0 { Err("Negative number".to_string()) }
    else {
        let result: i32;
        if i == 0 {
            result = 1;
        } else {
            result = i * factorial(i - 1);
        }
        Ok(result)
    }
}

pub fn test_factorial() {
    println!("Factorial of 5 is {} expected 120", factorial(5));
    println!("Factorial of 10 is {} expected 3628800", factorial(10));
    println!("Factorial of 15 is {} expected 1307674368000", factorial(15));
}