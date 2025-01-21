fn main() {
    let mut arr: Vec<char> = vec!['B', 'o', 'b'];

    let convert_to_lowercaps = |s: &char| -> char { s.to_lowercase().next().unwrap() };

    for elem in arr.iter_mut() {
        *elem = convert_to_lowercaps(elem);
    }

    if two_pointer_palindrome(&arr) {
        println!("the array is a palindrome");
    } else {
        println!("the array is not a palindrome");
    }
}

fn two_pointer_palindrome(array: &[char]) -> bool {
    let mut left_array_index = 0;
    let mut right_array_index = array.len() - 1;

    while left_array_index < right_array_index {
        if array[left_array_index] != array[right_array_index] {
            return false;
        }
        left_array_index += 1;
        right_array_index -= 1;
    }
    true
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_two_pointer_palindrome() {
        let test_array: Vec<char> = vec!['B', 'O', 'B'];
        assert_eq!(two_pointer_palindrome(&test_array), true)
    }
    #[test]
    fn test_odd_length_palindrome() {
        let test_array: Vec<char> = vec!['B', 'O', 'B', 'O', 'B'];
        assert_eq!(two_pointer_palindrome(&test_array), true)
    }
    #[test]
    fn test_even_length_palindrome() {
        let test_array: Vec<char> = vec!['B', 'O', 'B', 'B', 'O', 'B'];
        assert_eq!(two_pointer_palindrome(&test_array), true)
    }

    #[test]
    fn test_not_palindrome() {
        let test_array: Vec<char> = vec!['B', 'O', 'B', 'O', 'B', 'O'];
        assert_ne!(two_pointer_palindrome(&test_array), true)
    }
}
