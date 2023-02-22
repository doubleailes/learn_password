extern crate rpassword;
use rpassword::read_password;
use sha2::{Digest, Sha512};
use std::io::Write;
use std::time::Instant;
extern crate confy;
#[macro_use]
extern crate serde_derive;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Start the training
    #[arg(short, long)]
    train: bool,

    /// Store a new password
    #[arg(short, long)]
    store: bool,

    /// Display config path
    #[arg(short, long)]
    path: bool,

    /// Config name
    #[arg(short, long, default_value_t = String::from("default-config"))]
    count: String,
}

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

fn get_app_name() -> String {
    "learn_password".to_string()
}

fn store_config(my_cfg: ConfyConfig) -> Result<(), confy::ConfyError> {
    confy::store(&get_app_name(), None, my_cfg)?;
    Ok(())
}

fn get_config() -> ConfyConfig {
    confy::load(&get_app_name(), None).unwrap()
}

fn get_conf_path() {
    let file = confy::get_configuration_file_path(&get_app_name(), None).unwrap();
    println!("{}", file.display());
}

fn store() -> Result<(), confy::ConfyError> {
    let my_cfg = ConfyConfig {
        password_hashed: insert_password(),
    };
    store_config(my_cfg)?;
    Ok(())
}

fn train() {
    let cfg: ConfyConfig = get_config();
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

fn main() {
    let args = Args::parse();
    if args.train {
        train();
    } else if args.store {
        #[allow(unused_must_use)]
        store();
    } else if args.path {
        get_conf_path();
    }
}
