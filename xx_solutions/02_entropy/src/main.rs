use rand::Rng;

/// Returns a random integer selected uniformly from the interval [lower_bound, upper_bound].
fn create_entropy(lower_bound: i32, upper_bound: i32) -> i32 {
    rand::rng().random_range(lower_bound..=upper_bound)
}

/// Returns a random floating point value selected uniformly from the interval
/// [lower_bound, upper_bound).
fn create_more_entropy(lower_bound: f64, upper_bound: f64) -> f64 {
    rand::rng().random_range(lower_bound..upper_bound)
}

fn main() {
    println!("Creating entropy...");
    for _ in 1..=3 {
        println!("  -> {}", create_entropy(1, 10));
    }
    println!();

    println!("Creating *more* entropy...");
    for _ in 1..=3 {
        println!("  -> {}", create_more_entropy(0.0, 100.0));
    }
}
