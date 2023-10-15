pub fn factorial(i: i64) -> Result<i64, String>{
    if i < 0 {
        return Err("Factorial of negative number doesn't exist".to_string());
    } else if i > 20 {
        return Err("Factorial of number greater than 20 doesn't fit in i64".to_string());
    }

    let result;
    if i == 1 || i == 0 {
        result = 1;
    } else {
        result = i * factorial(i - 1).unwrap();
    }

    Ok(result)
}

pub fn try_factorial() {
    println!("Factorial of 5 is {:?}", factorial(5));
    println!("Factorial of 10 is {:?}", factorial(10));
}