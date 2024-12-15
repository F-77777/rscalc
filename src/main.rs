use rand::Rng;
fn main() {
    let mut rng = rand::thread_rng();

    // Generate a random number between 1 and 100
    let random_number: u64 = rng.gen_range(9999999999999999..=99999999990);
    println!("Random number: {}", random_number);
}
