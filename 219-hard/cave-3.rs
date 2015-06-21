use std::io::BufRead;

// Meet-in-the-middle algorithm.

// Compute the weight of every possible combination of nuggets.
fn compute_weights(nuggets: &[f64]) -> Vec<f64> {
    let n = nuggets.len();
    let mut weights = vec![0.0; 1 << n];
    for i in 0..n {
        let start = 1 << i;
        for j in 0..start {
            weights[start + j] = nuggets[i] + weights[j];
        }
    }
    weights
}

fn main() {
    // read input
    let stdin = std::io::stdin();
    let mut input = stdin.lock().lines();
    let backpack: f64 = input.next().unwrap().unwrap().parse().unwrap();
    let n: u64 = input.next().unwrap().unwrap().parse().unwrap();
    let nuggets: Vec<f64> =
        (0..n).map(|_| input.next().unwrap().unwrap()
                            .parse::<f64>().unwrap()).collect();

    // split nuggets into two sets A and B
    let (a, b) = nuggets.split_at(n as usize / 2);

    // compute weights for all subsets of A and B
    let weights_a = compute_weights(a);
    let weights_b = compute_weights(b);

    // find the best combination of nuggets from A and B
    let mut best = (0, 0.0);
    for subset_a in 0..(1 << a.len()) {
        for subset_b in 0..(1 << b.len()) {
            let combined_weight = weights_a[subset_a] + weights_b[subset_b];
            if combined_weight <= backpack && combined_weight > best.1 {
                best = (subset_b << a.len() | subset_a, combined_weight);
            }
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
