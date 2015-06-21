use std::io::BufRead;

// Meet-in-the-middle algorithm using binary search.

fn read_int() -> usize {
    let stdin = std::io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    line.parse::<usize>().unwrap()
}

fn read_weight() -> usize {
    let stdin = std::io::stdin();
    let line = stdin.lock().lines().next().unwrap();
    let numbers: Vec<usize> =
        line.unwrap().split('.').map(|n| n.parse().unwrap()).collect();
    10_000_000 * numbers[0] + numbers[1]
}

fn print_weight(w: usize) {
    println!("{}.{:07}", w / 10_000_000,
                         w % 10_000_000);
}

// Compute the weight of every possible combination of nuggets.
fn compute_weights(nuggets: &[usize]) -> Vec<(usize, usize)> {
    let n = nuggets.len();
    let mut weights = Vec::with_capacity(1 << n);
    weights.push((0, 0));
    for i in 0..n {
        let start = 1 << i;
        for j in 0..start {
            let weight = nuggets[i] + weights[j].1;
            weights.push((start + j, weight));
        }
    }
    weights.sort_by(|x, y| x.1.cmp(&y.1));
    weights
}

// Find the nugget combination whose weight is closest to `limit`.
fn find_closest(limit: usize, weights: &[(usize, usize)]) -> (usize, usize) {
    let mut lo = 0;
    let mut hi = weights.len();
    while hi - lo > 1 {
        let mid = (lo + hi) / 2;
        if weights[mid].1 <= limit { lo = mid; } else { hi = mid; }
    }
    weights[lo]
}

fn main() {
    // read input
    let backpack = read_weight();
    let n = read_int();
    let nuggets: Vec<usize> = (0..n).map(|_| read_weight()).collect();

    // split nuggets into two sets A and B
    let (a, b) = nuggets.split_at(n as usize / 2);

    // compute weights for all subsets of A and B
    let weights_a = compute_weights(a);
    let weights_b = compute_weights(b);

    // find the best combination of nuggets from A and B
    let mut best = (0, 0);
    for wa in weights_a {
        if wa.1 > backpack {
            continue;
        }
        let wb = find_closest(backpack - wa.1, &weights_b);
        let combined_weight = wa.1 + wb.1;
        if combined_weight > best.1 {
            best = (wb.0 << a.len() | wa.0, combined_weight);
        }
    }

    // print results
    print_weight(best.1);
    for i in 0..n {
        if 1 << i & best.0 != 0 {
            print_weight(nuggets[i as usize]);
        }
    }
}
