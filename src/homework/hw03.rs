const W: u32 = 20;
const H: u32 = 10;

fn envelope(width: u32, height: u32) {
    let k: f32 = width as f32 / height as f32;
    let mut output = String::new();

    for y in 0..height {
        for x in 0..width {
            let is_hor = y == 0 || y == height - 1;
            let is_ver = x == 0 || x == width - 1;
            let is_diag = x == (y as f32 * k) as u32;
            let is_diag2 = x == width - 1 - (y as f32 * k) as u32;
            let show = is_hor || is_ver || is_diag || is_diag2;

            output.push(if show { '*' } else { ' ' });
        }
        output.push('\n');
    }

    print!("{}", output);
}

fn main() {
    envelope(W, H);
}
