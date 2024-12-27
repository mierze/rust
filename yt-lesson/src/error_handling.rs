fn main() {
    // error handling techniques
    //
    // enum OPTION<T> {
    //     Some(T),
    //     None,
    // }

    // approach 2
    // enum RESULT<T, E> {
    //     Ok(T),
    //     Err(E),
    //
    // Error handling techniques

    // Custom enums can be defined if necessary, but we are using standard enums here.

    // Function using Option for error handling
    fn divide(numerator: f64, denominator: f64) -> Option<f64> {
        if denominator == 0.0 {
            None
        } else {
            Some(numerator / denominator)
        }
    }

    // Function using Result for error handling
    fn divide_r(num: f64, nom: f64) -> Result<f64, String> {
        if nom == 0.0 {
            Err("cannot divide by 0".to_string())
        } else {
            Ok(num / nom)
        }
    }

    // Using divide (Option-based)
    let result = divide(12.0, 4.2);
    match result {
        Some(x) => println!("opt_res: {x}"),
        None => println!("cannot divide by zero"),
    }

    // Using divide_r (Result-based)
    match divide_r(10.0, 2.0) {
        Ok(result) => println!("res_res: {}", result),
        Err(e) => println!("error: {}", e),
    }
}
