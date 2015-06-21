use std::io::BufRead;

// Dynamic programming solution.

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

fn main() {
    // read input
    let backpack = read_weight();
    let n = read_int();
    let nuggets: Vec<usize> = (0..n).map(|_| read_weight()).collect();

    // see which weights are possible
    let mut achievable = vec![0u32; backpack+1];
    achievable[0] = n as u32;
    for i in 0..n {
        let nugget = nuggets[i];
        let mut j = backpack;
        while j >= nugget {
            if achievable[j] == 0 &&
               achievable[j - nugget] != 0 {
                achievable[j] = i as u32 + 1;
            }
            j -= 1;
        }
    }

    // print the best nugget combination
    let mut i = backpack;
    while i > 0 && achievable[i] == 0 { i -= 1; }
    print_weight(i);
    while i != 0 {
        let nugget = nuggets[achievable[i] as usize - 1];
        print_weight(nugget);
        i -= nugget;
    }
}
