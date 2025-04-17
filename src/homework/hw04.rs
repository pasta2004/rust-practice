const HEIGHT: usize = 19;

fn main() {
    let mut output = String::new();
    let half = HEIGHT / 2;

    for i in 0..=half {
        for _ in 0..(half - i) {
            output += " ";
        }
        for _ in 0..(2 * i + 1) {
            output += "*";
        }
        output += "\n";
    }

    for i in (0..half).rev() {
        for _ in 0..(half - i) {
            output += " ";
        }
        for _ in 0..(2 * i + 1) {
            output += "*";
        }
        output += "\n";
    }

    println!("{}", output);
}
