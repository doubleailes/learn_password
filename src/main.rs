extern crate rpassword;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rpassword::read_password;
use std::io::Write;
use std::time::Instant;
extern crate confy;
#[macro_use]
extern crate serde_derive;
use clap::Parser;
use rand_core::OsRng;

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
    #[arg(short, long)]
    name: Option<String>,
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

fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt_string = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt_string)?;
    Ok(hash.to_string())
}

#[test]
fn test_hash_password() {
    let password = "toto";
    let hash = hash_password(password).unwrap();
    assert!(check_password(password, &hash).unwrap());
}

fn check_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let argon2: Argon2<'_> = Argon2::default();
    let parsed_hash: PasswordHash<'_> = PasswordHash::new(hash)?;
    let result: Result<(), argon2::password_hash::Error> =
        argon2.verify_password(password.as_bytes(), &parsed_hash);
    Ok(result.is_ok())
}

#[test]
fn test_check_password() {
    let password = "toto";
    let hash = hash_password(password).unwrap();
    assert!(check_password(password, &hash).unwrap());
}

fn input_password() -> Result<String, argon2::password_hash::Error> {
    print!("Type a password: ");
    std::io::stdout().flush().unwrap();
    let base_password = read_password().unwrap();
    let result: String = hash_password(&base_password)?;
    Ok(result)
}

fn get_app_name() -> String {
    "learn_password".to_string()
}

#[test]
fn test_get_app_name() {
    assert_eq!(get_app_name(), "learn_password".to_string());
}

fn store_config(my_cfg: ConfyConfig, config_name: Option<String>) -> Result<(), confy::ConfyError> {
    confy::store(&get_app_name(), config_name.as_deref(), my_cfg)?;
    Ok(())
}

fn get_config(config_name: Option<String>) -> ConfyConfig {
    confy::load(&get_app_name(), config_name.as_deref()).unwrap()
}

#[test]
fn test_get_config() {
    let my_cfg = ConfyConfig {
        password_hashed: "prout".to_string(),
    };
    store_config(my_cfg, None).unwrap();
    let cfg: ConfyConfig = get_config(None);
    assert_eq!(cfg.password_hashed, "prout".to_string());
}

fn get_conf_path(config_name: Option<String>) {
    let file: std::path::PathBuf =
        confy::get_configuration_file_path(&get_app_name(), config_name.as_deref()).unwrap();
    println!("{}", file.display());
}

fn store(config_name: Option<String>) -> Result<(), confy::ConfyError> {
    let my_cfg: ConfyConfig = ConfyConfig {
        password_hashed: input_password().expect("Error while hashing password"),
    };
    store_config(my_cfg, config_name)?;
    Ok(())
}

fn train(config_name: Option<String>) -> Result<(), argon2::password_hash::Error> {
    let cfg: ConfyConfig = get_config(config_name);
    let mut count: u16 = 0;
    let start: Instant = Instant::now();
    while check_password(&input_password()?, &cfg.password_hashed).unwrap() {
        count += 1;
    }
    let duration: std::time::Duration = start.elapsed();
    println!("You score {} in a row in {:?}", count, duration);
    Ok(())
}

fn main() {
    let args = Args::parse();
    if args.train {
        let _ = train(args.name);
    } else if args.store {
        let _ = store(args.name);
    } else if args.path {
        get_conf_path(args.name);
    }
}
