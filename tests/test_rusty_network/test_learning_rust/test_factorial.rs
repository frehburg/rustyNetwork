#[cfg(test)]
mod test_factorial {
    use rusty_network::rusty_network::learning_rust::factorial::factorial;

    #[test]
    fn test_factorial() {
        let result = 5 - 3;
        assert!(result > 0);
        let factorial_5: i64 = 120;
        let factorial_10: i64 = 3628800;
        let factorial_20: i64 = 2432902008176640000;
        assert_eq!(factorial(5).unwrap(), factorial_5);
        assert_eq!(factorial(10).unwrap(), factorial_10);
        assert_eq!(factorial(20).unwrap(), factorial_20);

        let result_negative = factorial(-1);
        let result_too_large = factorial(21);
        assert!(result_negative.is_err());
        assert!(result_too_large.is_err());
    }
}