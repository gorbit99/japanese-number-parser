use itertools::Itertools;

use crate::number_parts::{DIGITS, FINANCIAL_SEPARATORS};

use super::decimal::break_up_word;

pub fn parse_financial(japanese: &str) -> String {
    let (whole, decimal) = break_up_word(japanese);

    let mut chars = whole.chars().rev().peekable();
    let mut result = Vec::new();
    let mut last_was_digit = false;

    while let Some(c) = chars.next() {
        if let Some(digit) = DIGITS.get(&c) {
            result.push(digit.to_string());
            last_was_digit = true;
            continue;
        }
        last_was_digit = false;

        if let Some(next_char) = chars.peek() {
            let potential_separator = format!("{}{}", next_char, c);
            if let Some(power) = FINANCIAL_SEPARATORS.get(&potential_separator.as_str()) {
                result.resize(*power as usize, "0".to_string());
                chars.next();
                continue;
            }
        }

        let potential_separator = c.to_string();
        if let Some(power) = FINANCIAL_SEPARATORS.get(&potential_separator.as_str()) {
            result.resize(*power as usize, "0".to_string());
            //continue;
        }
    }

    if !last_was_digit {
        result.push("1".to_string());
    }

    result.iter().rev().join("") + decimal.as_str()
}
