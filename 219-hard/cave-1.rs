use std::io::BufRead;

// Naive solution: try every possible combination of nuggets.

fn main() {
    // read input
    let stdin = std::io::stdin();
    let mut input = stdin.lock().lines();
    let backpack: f64 = input.next().unwrap().unwrap().parse().unwrap();
    let n: u64 = input.next().unwrap().unwrap().parse().unwrap();
    let nuggets: Vec<f64> =
        (0..n).map(|_| input.next().unwrap().unwrap()
                            .parse::<f64>().unwrap()).collect();

    // try all combinations to find the best one
    let mut best = (0, 0.0);
    for bit_pattern in 1..(1 << n) {
        let mut total_weight = 0.0;
        for i in 0..n {
            total_weight += nuggets[i as usize] *
                            (((1 << i & bit_pattern) >> i) as f64);
        }
        if total_weight <= backpack && total_weight > best.1 {
            best = (bit_pattern, total_weight);
        }
    }

    // print results
    println!("{:.*}", 7, best.1);
    for i in 0..n {
        if 1 << i & best.0 != 0 {
            println!("{:.*}", 7, nuggets[i as usize]);
        }
    }
}
