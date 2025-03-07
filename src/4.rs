use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let a = rng.gen_range(1, 10);
    let b = rng.gen_range(1, 10);
    println!("{} + {} =", a, b);
    match a.cmp(&b) {
        Ordering::Less => println!("{}", a + b),
        Ordering::Greater => println!("{}", b + a),
        Ordering::Equal => println!("{}", a),
    }
}
