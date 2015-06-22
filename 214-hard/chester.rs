use std::io::BufRead;

struct Point { x: f64, y: f64, }

fn square(x: f64) -> f64 { x*x }

impl Point {
    fn distance(&self, p: &Point) -> f64 {
        self.distance_squared(p).sqrt()
    }
    fn distance_squared(&self, p: &Point) -> f64 {
        square(self.x - p.x) + square(self.y - p.y)
    }
}

fn read_input() -> Vec<Point> {
    let stdin = std::io::stdin();
    let mut input = stdin.lock().lines();
    let n_treats = input.next().unwrap().unwrap().parse().unwrap();
    let mut treats = Vec::with_capacity(n_treats);
    for _ in 0..n_treats {
        let numbers: Vec<f64> = input.next().unwrap().unwrap().split(' ')
                                     .filter_map(|s| s.parse().ok())
                                     .collect();
        treats.push(Point { x: numbers[0], y: numbers[1] } );
    }
    treats
}

// Remove the point closest to `p` and return it.
fn pop_closest(p: &Point, ps: &mut Vec<Point>) -> Option<Point> {
    if ps.is_empty() { return None; }
    let len = ps.len();
    let mut closest = (0, p.distance_squared(&ps[0]));
    for i in 1..len {
        let d = p.distance_squared(&ps[i]);
        if d < closest.1 {
            closest = (i, d);
        }
    }
    Some(ps.swap_remove(closest.0))
}

fn main() {
    let mut treats = read_input();
    let mut position = Point { x:0.5, y:0.5 };
    let mut total_distance = 0f64;
    while let Some(next_position) = pop_closest(&position, &mut treats) {
        total_distance += position.distance(&next_position);
        position = next_position;
    }
    println!("{:.16}", total_distance);
}
