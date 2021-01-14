pub mod math;
pub mod shamir;

use std::env::Args;
use std::error::Error;

// configuration when working in encrypt (c) mode.
pub struct EncryptConfig {
    total_evals: u32,
    min_required_evals: u32,
    shares_path: String,
    plaintext: String,
    password: String,
}

// configuration when working in decrypt (d) mode.
pub struct DecryptConfig {
    encrypted_file: String,
    keys_file: String,
}

/// This enum represents a configuration to execute
/// the cipher.
///
/// Since the ciper can either work in encrypt or decrypt
/// mode, this enum contains to options Encrypt and Decrypt
/// that wraps each of these modes.
pub enum Config {
    Encrypt(EncryptConfig),
    Decrypt(DecryptConfig),
}

impl Config {
    /// Creates a new configuration instance by parsing
    /// the arguments provided.
    ///
    /// The structure of the arguments should be the following
    ///
    /// if encrypting:
    ///
    /// $  <program-name> c <plaintext> <keys-path> <n> <r>
    ///
    /// where:
    ///
    /// - plaintext is the path to the file containing the message to encrypt.
    /// - keys-path is the path to the file where the keys will be stored.
    /// - n is the total number of shares to generate and must be grater than 2.
    /// - r is the minimun required number of shares to decrypt the message
    /// (1 < t <= n).
    ///
    /// if decrypting:
    ///
    /// $ <program-name> d <encrypted-text> <
    ///
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        Err("not implemented") // TODO
    }
}

/// Runs the program, encrypting or decrypting the file according
/// to the configuration passed.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    Ok(()) // TODO
}
