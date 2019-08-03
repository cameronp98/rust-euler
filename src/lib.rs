pub mod prime;

/// determine if number `n` is a palindrome
pub fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_palindrome_works() {
        use super::is_palindrome;

        let palindromes = [1, 22, 333, 434, 5665, 678876, 23455432, 1234554321];

        assert!(palindromes.iter().all(|&n| is_palindrome(n)));
    }
}