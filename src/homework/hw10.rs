fn is_palindrome(x: u32) -> bool {
    let s = x.to_string();
    let reversed: String = s.chars().rev().collect();
    s == reversed
}

fn main() {
    let num = 121;
    println!("{} is palindrome? {}", num, is_palindrome(num));
}

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
