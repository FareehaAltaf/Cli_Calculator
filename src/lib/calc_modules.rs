pub mod input {

    use std::io;
    pub fn input<T: std::str::FromStr>() -> Option<T> {
        // Option used for when parsing fails without panicking.
        loop {
            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                eprintln!("Error reading input. Please try again."); //print immediately after the error message is generated when the user enters an empty string
                continue;
            }

            let trimmed_input = input.trim();
            if trimmed_input.is_empty() {
                eprintln!("Input is empty. Please enter a value.");
                continue;
            }

            match trimmed_input.parse::<T>() {
                Ok(value) => return Some(value), // only when value is not empty and is of type T
                Err(_) => {
                    eprintln!("Invalid input. Please try again.");
                    continue;
                }
            }
        }
    }

    pub fn input_op() -> Option<char> {
        loop {
            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                eprintln!("Error reading input. Please try again.");
                continue;
            }

            let trimmed_input = input.trim();
            if trimmed_input.is_empty() {
                eprintln!("Input is empty. Please enter a value.");
                continue;
            }

            match trimmed_input.parse::<char>() {
                Ok(value) if value == '+' || value == '-' || value == '*' || value == '/' => {
                    return Some(value)
                }
                _ => {
                    eprintln!("Invalid input. Please try again.");
                    continue;
                }
            }
        }
    }
}

pub fn calculation(a: f32, b: f32, operation: char) -> Result<String, String> {
    match operation {
        '+' => Ok(format!("Answer: {:.1} + {:.1} = {:.1}", a, b, a + b)),
        '-' => Ok(format!("Answer: {:.1} - {:.1} = {:.1}", a, b, a - b)),
        '*' => Ok(format!("Answer: {:.1} * {:.1} = {:.1}", a, b, a * b)),
        '/' => {
            if b == 0.0 {
                Err(String::from("Division by zero is not allowed."))
            } else {
                Ok(format!("Answer: {:.1} / {:.1} = {:.1}", a, b, a / b))
            }
        }
        _ => Err(String::from("Invalid operation.")),
    }
}
//?                                                       UNIT TESTING
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let result = calculation(2.0, 3.0, '+');
        assert_eq!(result, Ok("Answer: 2.0 + 3.0 = 5.0".to_string()));
    }

    #[test]
    fn test_subtraction() {
        let result = calculation(5.0, 3.0, '-');
        assert_eq!(result, Ok("Answer: 5.0 - 3.0 = 2.0".to_string()));
    }

    #[test]
    fn test_multiplication() {
        let result = calculation(5.0, 3.0, '*');
        assert_eq!(result, Ok("Answer: 5.0 * 3.0 = 15.0".to_string()));
    }

    #[test]
    fn test_division() {
        let result = calculation(5.0, 5.0, '/');
        assert_eq!(result, Ok("Answer: 5.0 / 5.0 = 1.0".to_string()));
    }
}
