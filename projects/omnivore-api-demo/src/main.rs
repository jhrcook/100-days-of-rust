use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Secrets {
    omnivore_api_key: String,
}

fn main() {
    let secrets = get_omnivore_api_key().expect("Could not locate Omnivore API key.");
    println!("Omnivore API key: {}", secrets.omnivore_api_key);


}

fn get_omnivore_api_key() -> Result<Secrets, serde_yaml::Error> {
    let f = std::fs::File::open("secrets.yaml").expect("Could not open file.");
    let secrets: Secrets = serde_yaml::from_reader(f).expect("Could not read values.");
    return Ok(secrets);
}
