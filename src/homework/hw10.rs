fn is_palindrome(x: u32) -> bool {
    let s = x.to_string();
    s.chars().eq(s.chars().rev())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
       let data = [
           (123, false),
           (121, true),
           (1221, true),
       ];

       data.iter().for_each(|(n, exp)| {
           assert_eq!(is_palindrome(*n), *exp);
       });
    }
}
