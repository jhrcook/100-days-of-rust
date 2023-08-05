// Following the examples from the Rust Cookbook.
// https://rust-lang-nursery.github.io/rust-cookbook/concurrency.html

use rand_distr::{Distribution, Poisson};

fn main() {
    let between = Poisson::new(1.0).unwrap();
    let mut rng = rand::thread_rng();

    let arr: Vec<f64> = (0..10000).map(|_| between.sample(&mut rng)).collect();
    let max = find_max(&arr);

    if let Some(max_val) = max {
        println!("Max value: {}", max_val)
    } else {
        println!("No maximum found.")
    }
}

fn find_max(arr: &[f64]) -> Option<f64> {
    const THRESHOLD: usize = 2;

    if arr.len() <= THRESHOLD {
        // return arr.i
        return arr
            .into_iter()
            .cloned()
            .max_by(|a, b| a.partial_cmp(b).unwrap());
        // return arr.iter().cloned().max();
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at(mid);

    crossbeam::scope(|s| {
        let thread_l = s.spawn(|_| find_max(left));
        let thread_r = s.spawn(|_| find_max(right));

        let max_l = thread_l.join().unwrap()?;
        let max_r = thread_r.join().unwrap()?;

        Some(max_l.max(max_r))
    })
    .unwrap()
}

// fn find_max(arr: )
