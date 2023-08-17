use rand::{distributions::Distribution, distributions::Uniform, Rng};
use rand_distr::{Alphanumeric, Normal, Standard};

fn main() {
    let funcs = [
        generate_random_numbers,
        generate_random_numbers_within_a_range,
        generate_random_numbers_from_a_distribution,
        generate_random_values_of_a_custom_type,
        generate_random_alphanumeric_passwords,
    ];
    for func in funcs.iter() {
        func();
    }
}

fn generate_random_numbers() {
    let mut rng = rand::thread_rng();
    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8 and u16: {}, {}", n1, n2);
    println!("Random i32: {}", rng.gen::<i32>());
}

fn generate_random_numbers_within_a_range() {
    let mut rng = rand::thread_rng();
    println!("Random integer within [0, 10): {}", rng.gen_range(0..10));
    println!("Random float within [0, 10): {}", rng.gen_range(0.0..10.0));

    print!("Sampling from [0, 10] until 10 is found: ");
    let uniform_dist = Uniform::from(0..=10);
    loop {
        let sample = uniform_dist.sample(&mut rng);
        print!("{}", sample);
        if sample == 10 {
            break;
        }
        print!(" ");
    }
    println!();
}

fn generate_random_numbers_from_a_distribution() {
    let mu = 0.0;
    let sigma = 2.0;
    for n in [2, 5, 10, 100, 100_000] {
        let samples: Vec<f64> = (0..n).map(|_| rnormal(mu, sigma)).collect();
        println!(
            "observed dist. after {} samples: N({:.3}, {:.3})",
            n,
            calc_mean(&samples),
            calc_standard_deviation(&samples)
        );
    }
}

fn rnormal(mu: f64, sigma: f64) -> f64 {
    let mut rng = rand::thread_rng();
    match Normal::new(mu, sigma) {
        Ok(normal) => normal.sample(&mut rng),
        Err(_) => panic!(
            "Could not create normal distribution of N({}, {}.",
            mu, sigma
        ),
    }
}

fn calc_mean(x: &Vec<f64>) -> f64 {
    x.iter().sum::<f64>() / x.len() as f64
}

fn calc_standard_deviation(x: &Vec<f64>) -> f64 {
    let mean = calc_mean(x);
    let diffs: Vec<f64> = x.iter().map(|i| (i - mean).exp2()).collect();
    let mean_diffs = calc_mean(&diffs);
    mean_diffs.sqrt()
}

#[allow(dead_code)]
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

fn generate_random_values_of_a_custom_type() {
    let mut rng = rand::thread_rng();
    let rand_point1: Point = rng.gen();
    println!("Random point 1: {:?}", rand_point1);
    let rand_point2 = rng.gen::<Point>(); // turbofish syntax!
    println!("Random point 2: {:?}", rand_point2);
}

fn generate_random_alphanumeric_passwords() {
    println!("password: {}", password_gen());
}

fn password_gen() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
}
