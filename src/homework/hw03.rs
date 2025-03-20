const WIDTH: usize = 10;
const HEIGHT: usize = 6;

fn main() {
    let mut output = String::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if x == 0 || x == WIDTH - 1 {
                output.push('*');
            } else if y == 0 || y == HEIGHT - 1 {
                output.push('*');
            } else if x == y || x == WIDTH - y - 1 {
                output.push('*');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }
    
    println!("{}", output);
}

/**********
 **      **
 * *    * *
 *  *  *  *
 *   **   *
 **********/
