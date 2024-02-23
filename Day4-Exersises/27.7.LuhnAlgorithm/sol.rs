pub fn luhn(cc_number: &str) -> bool {
    let mut sum =  0;
    let mut double = false;

    for c in cc_number.chars().rev() {
        if c.is_whitespace() {
            continue;
        }
        if let Some(digit) = c.to_digit(10) {
            if double {
                let double_digit = digit *  2;
                sum += if double_digit >  9 { double_digit -  9 } else { double_digit };
            } else {
                sum += digit;
            }
            double = !double;
        } else {
            return false; // Reject number with non-digit characters
        }
    }

    sum %  10 ==  0
}

use std::io;

fn main() {
    println!("Enter a credit card number:");

    let mut cc_number = String::new();
    io::stdin().read_line(&mut cc_number).expect("Failed to read line");

    let cc_number = cc_number.trim(); // Trim the input to remove any trailing newline characters

    if luhn(cc_number) {
        println!("The credit card number is valid.");
    } else {
        println!("The credit card number is invalid.");
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_number() {
        assert!(luhn("4263  9826  4026  9299"));
        assert!(luhn("4539  3195  0343  6467"));
        assert!(luhn("7992  7398  713"));
    }

    #[test]
    fn test_invalid_number() {
        assert!(!luhn("4223  9826  4026  9299"));
        assert!(!luhn("4539  3195  0343  6476"));
        assert!(!luhn("8273  1232  7352  0569"));
    }

    #[test]
    fn test_number_with_spaces() {
        assert!(luhn("4263  9826  4026  9299"));
        assert!(!luhn("4223  9826  4026  9299"));
    }

    #[test]
    fn test_number_with_non_digit_characters() {
        assert!(!luhn("4263-9826-4026-9299"));
        assert!(!luhn("4263  9826  4026  9299X"));
    }

    #[test]
    fn test_number_with_fewer_than_two_digits() {
        assert!(!luhn("1"));
        assert!(!luhn("12"));
    }
}