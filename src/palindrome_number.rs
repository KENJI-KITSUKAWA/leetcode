pub fn is_palindrome(x: i32) -> bool {
    let x = x;
    let mut y = 0;
    let mut z = x;
    while z > 0 {
        y = y * 10 + z % 10;
        z = z / 10;
    }
    x == y
}

pub fn is_palindrome2(x: i32) -> bool {
    let x = x.to_string();
    let y = x.chars().rev().collect::<String>();

    x == y
}

#[test]
fn test_is_palindrome() {
    assert_eq!(is_palindrome(121), true);
    assert_eq!(is_palindrome(-121), false);
    assert_eq!(is_palindrome(10), false);
    assert_eq!(is_palindrome(0), true);
    assert_eq!(is_palindrome(1), true);
    assert_eq!(is_palindrome(11), true);
    assert_eq!(is_palindrome(111), true);
    assert_eq!(is_palindrome(1111), true);
    assert_eq!(is_palindrome(11111), true);
    assert_eq!(is_palindrome(111111), true);
    assert_eq!(is_palindrome(1111111), true);
    assert_eq!(is_palindrome(11111111), true);
    assert_eq!(is_palindrome(111111111), true);

    assert_eq!(is_palindrome2(121), true);
    assert_eq!(is_palindrome2(-121), false);
    assert_eq!(is_palindrome2(10), false);
    assert_eq!(is_palindrome2(0), true);
    assert_eq!(is_palindrome2(1), true);
    assert_eq!(is_palindrome2(11), true);
    assert_eq!(is_palindrome2(111), true);
    assert_eq!(is_palindrome2(1111), true);
    assert_eq!(is_palindrome2(11111), true);
    assert_eq!(is_palindrome2(111111), true);
    assert_eq!(is_palindrome2(1111111), true);
    assert_eq!(is_palindrome2(11111111), true);
    assert_eq!(is_palindrome2(111111111), true);
}
