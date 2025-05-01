fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

#[test]
fn test_is_prime() {
    let test_data = [
        (0, false),
        (1, false),
        (2, true),
        (3, true),
        (4, false),
        (5, true),
        (10, false),
        (101, true),
    ];

    test_data
        .iter()
        .for_each(|&(number, expected)| assert_eq!(is_prime(number), expected));
}

fn main() {
    let num = 5;
    if is_prime(num) {
        println!("Число {} є простим!", num);
    } else {
        println!("Число {} не є простим.", num);
    }
}
