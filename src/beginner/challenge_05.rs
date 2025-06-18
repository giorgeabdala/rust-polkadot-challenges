use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
enum SimpleError {
    InvalidInput,
    OutOfRange
}

impl From<std::num::ParseIntError> for SimpleError {
    fn from(_value: ParseIntError) -> Self {
        SimpleError::InvalidInput
    }
}


fn safe_divide(a: i32, b:i32) -> Result<i32, SimpleError> {
    a.checked_div(b).ok_or(SimpleError::InvalidInput)
}

fn parse_positive(s: &str) -> Result<i32, SimpleError> {
    let num = s.parse::<i32>()?;

    if num <= 0 {
        Err(SimpleError::OutOfRange)
    } else {
        Ok(num)
    }
}


fn calculate_average(numbers: &[&str]) -> Result<i32, SimpleError> {
    if numbers.is_empty() { return Err(SimpleError::InvalidInput); }
    let mut sum = 0;
    for &num_str in numbers {
        let num = parse_positive(num_str)?;
        sum += num;
    }
    safe_divide(sum, numbers.len() as i32)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_divide_success() {
        assert_eq!(safe_divide(10, 2), Ok(5));
        assert_eq!(safe_divide(7, 3), Ok(2));
    }

    #[test]
    fn test_safe_divide_by_zero() {
        assert_eq!(safe_divide(10, 0), Err(SimpleError::InvalidInput));
    }

    #[test]
    fn test_parse_positive_success() {
        assert_eq!(parse_positive("42"), Ok(42));
        assert_eq!(parse_positive("1"), Ok(1));
    }

    #[test]
    fn test_parse_positive_invalid_string() {
        assert_eq!(parse_positive("abc"), Err(SimpleError::InvalidInput));
        assert_eq!(parse_positive("12.5"), Err(SimpleError::InvalidInput));
    }

    #[test]
    fn test_parse_positive_out_of_range() {
        assert_eq!(parse_positive("-5"), Err(SimpleError::OutOfRange));
        assert_eq!(parse_positive("0"), Err(SimpleError::OutOfRange));
    }

    #[test]
    fn test_calculate_average_success() {
        assert_eq!(calculate_average(&["10", "20"]), Ok(15)); 
        assert_eq!(calculate_average(&["3", "7"]), Ok(5));
    }

    #[test]
    fn test_calculate_average_parse_error() {
        assert_eq!(calculate_average(&["abc", "5"]), Err(SimpleError::InvalidInput));
    }

    #[test]
    fn test_calculate_average_range_error() {
        assert_eq!(calculate_average(&["-1", "5"]), Err(SimpleError::OutOfRange));
    }
    
    #[test]
    fn test_calculate_average_parse_error_second_number() {
        assert_eq!(calculate_average(&["5", "xyz"]), Err(SimpleError::InvalidInput));
    }

    #[test]
    fn test_calculate_average_range_error_second_number() {
        assert_eq!(calculate_average(&["5", "-2"]), Err(SimpleError::OutOfRange));
    }
}
