// apple-and-orange

fn countApplesAndOranges(
house_l: i32, house_r: i32,
apple_center: i32, orange_center: i32, 
apples: &[i32], oranges: &[i32]) {
    let mut apple_count:i32 = 0;
    for &delta in apples {
        let pos:i32 = apple_center + delta;
        if pos >= house_l && pos <= house_r { apple_count += 1; }
    }

    let mut orange_count:i32 = 0;
    for &delta in oranges {
        let pos:i32 = orange_center + delta;
        if pos >= house_l && pos <= house_r { orange_count += 1; }
    }

    println!("{apple_count}\n{orange_count}");
}
