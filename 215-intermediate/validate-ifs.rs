use std::io::BufRead;

// slower version using `if` instead of boolean logic

// read a pair of numbers from standard input
fn read_pair() -> (usize, usize) {
    let stdin = std::io::stdin();
    let line = stdin.lock().lines().next().unwrap();
    let numbers: Vec<usize> =
        line.unwrap().split(' ').map(|n| n.parse().unwrap()).collect();
    (numbers[0], numbers[1])
}

// iterator over all boolean vectors of size `n`
fn bool_permutations(n: usize) -> BoolPermutations {
    BoolPermutations { current: vec![false; n] }
}
struct BoolPermutations { current: Vec<bool> }
impl Iterator for BoolPermutations {
    type Item = Vec<bool>;
    fn next(&mut self) -> Option<Vec<bool>> {
        let previous = self.current.clone();
        let mut carry = true;
        for b in self.current.iter_mut() {
            let new_b = *b ^ carry;
            carry = *b && carry;
            *b = new_b;
        }
        if carry { None } else { Some(previous) }
    }
}

// apply sorting network to `input`
fn sort(input: &mut Vec<bool>, comparators: &Vec<(usize, usize)>) {
    for c in comparators {
        let (a, b) = *c;
        if input[a] && !input[b] {
            input[a] = false;
            input[b] = true;
        }
    }
}

// check if the vector is sorted (first `false`, then `true`)
fn is_sorted(input: &Vec<bool>) -> bool {
    let mut must_be_true = false;
    for i in input {
        if must_be_true && !*i {
            return false;
        }
        must_be_true = *i;
    }
    true
}

fn main() {
    // read input
    let (n_wires, n_comparators) = read_pair();
    let mut comparators = Vec::new();
    for _ in 0..n_comparators {
        comparators.push(read_pair());
    }

    // see if the network is valid
    for mut sequence in bool_permutations(n_wires) {
        sort(&mut sequence, &comparators);
        if !is_sorted(&sequence) {
            println!("Invalid network");
            return;
        }
    }
    println!("Valid network");
}
