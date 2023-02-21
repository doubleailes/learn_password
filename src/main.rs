extern crate rpassword;
use rpassword::read_password;
use sha2::{Digest, Sha512};
use std::io::Write;
use std::time::Instant;
extern crate confy;
#[macro_use]
extern crate serde_derive;

#[derive(Debug, Serialize, Deserialize)]
struct ConfyConfig {
    password_hashed: String,
}

impl Default for ConfyConfig {
    fn default() -> Self {
        ConfyConfig {
            password_hashed: "prout".to_string(),
        }
    }
}

fn insert_password() -> String {
    print!("Type a password: ");
    std::io::stdout().flush().unwrap();
    let base_password = read_password().unwrap();
    // create a Sha256 object
    let mut hasher = Sha512::new();
    // write input message
    hasher.update(base_password);
    // read hash digest and consume hasher
    let result = hasher.finalize();
    format!("{result:x}")
}

fn main() {
    let cfg: ConfyConfig = confy::load("learn_password", None).unwrap();
    let mut count: u8 = 0;
    let start = Instant::now();
    let mut password = insert_password();
    while password == cfg.password_hashed {
        count += 1;
        password = insert_password();
    }
    let duration = start.elapsed();
    println!("You score {} in a row in {:?}", count, duration)
}
