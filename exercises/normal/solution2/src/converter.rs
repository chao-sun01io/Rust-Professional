pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // Split the input string into the number and the base it is currently in
    let parts: Vec<&str> = num_str.split('(').collect();
    if parts.len() != 2 {
        panic!("Invalid input format");
    }

    // Parse the base the number is currently in
    let from_base: u32 = parts[1].trim_end_matches(')').parse().expect("Invalid base format");
    
    // Parse the number in the given base
    let number: i32 = i32::from_str_radix(parts[0], from_base).expect("Invalid number format");

    // Convert the number to the desired base
    decimal_to_base(number, to_base)
}

fn decimal_to_base(mut num: i32, to_base: u32) -> String {
    if num == 0 {
        return "0".to_string();
    }

    let mut result = String::new();
    let is_negative = num < 0;
    if is_negative {
        num = -num; // Handle negative numbers
    }

    while num > 0 {
        let remainder = num % to_base as i32;
        let digit = match remainder {
            0..=9 => char::from_digit(remainder as u32, 10).unwrap(),
            _ => char::from_u32(remainder as u32 - 10 + 'a' as u32).unwrap(),
        };
        result.push(digit);
        num /= to_base as i32;
    }

    if is_negative {
        result.push('-');
    }

    result.chars().rev().collect()
}