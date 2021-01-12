mod shamir;

use std::error::Error;

pub struct EncryptConfig {
    pub total_evals: u32,
    pub min_required_evals: u32,
    pub shares_filepath: String,
    pub file_to_encrypt: String,
    pub password: String,
}

pub struct DecryptConfig {
    pub encrypted_file: String,
    pub keys_file: String,
}

pub enum Config {
    Encrypt(EncryptConfig),
    Decrypt(DecryptConfig),
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    Ok(()) // TODO
}
