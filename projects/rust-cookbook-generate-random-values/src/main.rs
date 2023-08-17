use rand::Rng;

fn main() {
    generate_random_numbers();
}

fn generate_random_numbers() {
    let mut rng = rand::thread_rng();
    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8 and u16: {}, {}", n1, n2);
    println!("Random i32: {}", rng.gen::<i32>());
}
