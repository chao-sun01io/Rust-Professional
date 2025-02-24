/*
    Palindrome Check
    Given a string, check if it is a palindrome (i.e., it reads the same forward and backward).
    The solution should ignore case differences and non-alphabetical characters.

    You need to implement the function `is_palindrome(s: String) -> bool`.
    The function should return `true` if the string is a palindrome, and `false` otherwise.

    Hint: Consider normalizing the string by converting it to lowercase and removing non-alphabetical characters before checking.
*/

use std::fmt::{self, Display, Formatter};

pub fn is_palindrome(s: String) -> bool {
    // TODO: Implement the logic to check if the string is a palindrome

    // removing non-alphabetical characters and transform it to lowercases
    // let normalised_string: String = s
    //     .to_lowercase()
    //     .chars()
    //     .filter(|c| c.is_alphabetic())
    //     .collect();
    let mut normalised_string = s.to_lowercase();
    normalised_string.retain(|c| c.is_alphabetic());

    // TODO1 : use retain to get a sort of memory efficiency
    // TODO2 : handle the case and non-alphabetical character on the fly

    // Method 1
    // Use two pointers until they meet
    let chars: Vec<char> = normalised_string.chars().collect();
    let mut l = 0;
    let mut r = normalised_string.len().saturating_sub(1);
    while l < r {
        if chars[l] != chars[r] {
            return false;
        }
        l += 1;
        r -= 1;
    }
    true

    // Method 2: functional programming
    // let chars = normalised_string.chars();
    // let chars_rev = normalised_string.chars().rev();

    // chars
    //     .zip(chars_rev)
    //     .take(normalised_string.len() / 2)
    //     .all(|(a, b)| a == b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_str() {
        let s = String::new();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_palindrome_1() {
        let s = "A man, a plan, a canal, Panama".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_palindrome_2() {
        let s = "Racecar".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_palindrome_3() {
        let s = "Hello, World!".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, false);
    }

    #[test]
    fn test_palindrome_4() {
        let s = "No 'x' in Nixon".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_palindrome_5() {
        let s = "Was it a car or a cat I saw?".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }
}
