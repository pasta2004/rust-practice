use rand::Rng;

fn count_permutation(shipments: &Vec<u32>) -> isize {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    if total % n != 0 {
        return -1; 
    }

    let average = total / n;
    let mut moves = 0;

    for &weight in shipments {
        if weight > average {
            moves += weight - average;
        }
    }

    moves as isize
}

fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let avg = rng.gen_range(1..=10);
    let mut shipments = vec![avg; n];

    for _ in 0..(n / 2) {
        let from = rng.gen_range(0..n);
        let to = rng.gen_range(0..n);
        if shipments[from] > 0 && from != to {
            let delta = rng.gen_range(1..=shipments[from].min(3));
            shipments[from] -= delta;
            shipments[to] += delta;
        }
    }

    shipments
}

fn main() {
    let examples = vec![
        vec![8, 2, 2, 4, 4],
        vec![9, 3, 7, 2, 9],
        vec![1, 1, 1, 1, 6], 
    ];

    for (i, shipment) in examples.iter().enumerate() {
        println!("Example {}:", i + 1);
        println!("Shipments: {:?}", shipment);
        let answer = count_permutation(shipment);
        if answer == -1 {
            println!("Result: Impossible to balance\n");
        } else {
            println!("Minimum moves needed: {}\n", answer);
        }
    }
  
    let generated = gen_shipments(6);
    println!("Generated shipments: {:?}", generated);
    println!(
        "Moves needed to balance: {}\n",
        count_permutation(&generated)
    );
}
