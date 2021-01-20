use std::convert::TryInto;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::Path;

use clap::ArgMatches;
use rpassword;

use crypto::{Cipher, Share};
use error::*;

pub mod crypto;
pub mod error;
pub mod math;

// configuration when working in encrypt (c) mode.
pub struct EncryptConfig {
    pub total_evals: usize,
    // TODO make this private again.
    pub min_required_evals: usize,
    pub input_file: String,
    pub output_file: String,
    pub password: String,
}

// configuration when working in decrypt (d) mode.
pub struct DecryptConfig {
    pub encrypted_file: String,
    pub shares_file: String,
}

/// This enum represents a configuration to execute
/// the cipher.
///
/// Since the cipher can either work in encrypt or decrypt
/// mode, this enum contains to options Encrypt and Decrypt
/// that wraps each of these modes.
pub enum Config {
    Encrypt(EncryptConfig),
    Decrypt(DecryptConfig),
}

impl Config {
    /// Creates a new configuration instance by parsing
    /// the arguments provided.
    pub fn new(args: ArgMatches) -> Result<Config, Box<dyn Error>> {
        match args.subcommand() {
            ("c", Some(c_matches)) => {
                let password = rpassword::read_password_from_tty(Some("Password to encrypt: "))?;
                let total_evals = c_matches.value_of("N").unwrap().parse()?;
                if total_evals <= 2 {
                    return Err(Box::new(ArgumentError("N must be greater than 2".into())));
                }
                let min_required_evals = c_matches.value_of("K").unwrap().parse()?;
                if min_required_evals <= 0 || min_required_evals > total_evals {
                    return Err(Box::new(ArgumentError(
                        "K must be greater than 0 and not greater than N".into(),
                    )));
                }
                Ok(Config::Encrypt(EncryptConfig {
                    input_file: String::from(c_matches.value_of("INPUT").unwrap()),
                    output_file: String::from(c_matches.value_of("OUTPUT_NAME").unwrap()),
                    total_evals,
                    min_required_evals,
                    password,
                }))
            }
            ("d", Some(d_matches)) => Ok(Config::Decrypt(DecryptConfig {
                shares_file: String::from(d_matches.value_of("SHARES").unwrap()),
                encrypted_file: String::from(d_matches.value_of("ENCRYPTED_FILE").unwrap()),
            })),
            _ => panic!(),
        }
    }
}

/// Runs the program, encrypting or decrypting the file according
/// to the configuration passed.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config {
        Config::Encrypt(config) => run_encrypt(config),
        Config::Decrypt(config) => run_decrypt(config),
    }
}

// Runs the program in encrypt mode
fn run_encrypt(config: EncryptConfig) -> Result<(), Box<dyn Error>> {
    let cipher = Cipher::new(&config.password);
    encrypt_file(&config, &cipher)?;
    save_shares(&config, &cipher)?;
    Ok(())
}

// Reads, encrypts and saves the result
fn encrypt_file(config: &EncryptConfig, cipher: &Cipher) -> Result<(), Box<dyn Error>> {
    let mut file_content = fs::read(&config.input_file)?;
    cipher.encrypt(&mut file_content)?;
    let output_file = create_file(format!("./{}.aes", config.output_file))?;
    let mut writer = BufWriter::new(output_file);
    let original_name = Path::new(&config.input_file)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    // write name as first line
    writer.write_all(original_name.as_bytes())?;
    writer.write_all(b"\n")?;
    // write encrypted file
    writer.write_all(&file_content)?;
    writer.flush()?;
    Ok(())
}

// Save the shares in the disk
fn save_shares(config: &EncryptConfig, cipher: &Cipher) -> Result<(), Box<dyn Error>> {
    let shares_file = create_file(format!("./{}.frg", config.output_file))?;
    let mut writer = BufWriter::new(shares_file);
    for (x, y) in cipher.split_key(config.total_evals, config.min_required_evals) {
        writer.write(x.as_bytes())?;
        writer.write(b":")?;
        writer.write(y.as_bytes())?;
        writer.write(b"\n")?;
    }
    writer.flush()?;
    Ok(())
}

// creates a file failing if already exists
fn create_file(path: String) -> Result<fs::File, std::io::Error> {
    OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(path)
}

// Runs the program in decrypt mode
fn run_decrypt(config: DecryptConfig) -> Result<(), Box<dyn Error>> {
    let cipher = Cipher::from_shares(recover_key(&config)?.into_iter())?;
    decrypt_file(&config, &cipher)?;
    Ok(())
}

// Recovers the key from the shares file
fn recover_key(config: &DecryptConfig) -> Result<Vec<Share>, Box<dyn Error>> {
    let reader = BufReader::new(File::open(&config.shares_file)?);
    Ok(reader
        .lines()
        .map::<Result<Share, Box<dyn Error>>, _>(|l| {
            let line = l?;
            let eval: Vec<&str> = line.split(":").collect();
            let x = eval[0].trim().to_string();
            let y = eval[1].trim().to_string();
            if x.len() == 0 || y.len() == 0 || eval.len() != 2 {
                return Err(Box::new(CorruptFileError(
                    "fragments file is corrupt".into(),
                )));
            }
            Ok((x, y))
        })
        .collect::<Result<Vec<Share>, _>>()?)
}

// decrypts the file and writes the result in disk
fn decrypt_file(config: &DecryptConfig, cipher: &Cipher) -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open(&config.encrypted_file)?);
    // read original name
    let mut original_name = String::new();
    reader.read_line(&mut original_name)?;
    // remove the newline char
    let original_name = original_name.replace("\n", "");
    // Read rest of file
    let file_length = fs::metadata(&config.encrypted_file)?.len();
    let mut file_content = Vec::with_capacity(file_length.try_into()?);
    reader.read_to_end(&mut file_content)?;
    // decrypt file
    cipher.decrypt(&mut file_content)?;
    // save file
    let output_file = create_file(format!("{}", original_name))?;
    let mut writer = BufWriter::new(output_file);
    writer.write_all(&file_content)?;
    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integration() {
        let encrypt_config = EncryptConfig {
            total_evals: 5,
            min_required_evals: 4,
            input_file: "test_data/msg1.txt".into(),
            output_file: "ciphered".into(),
            password: "secure password".into(),
        };
        let decrypt_config = DecryptConfig {
            shares_file: "ciphered.frg".into(),
            encrypted_file: "ciphered.aes".into(),
        };
        run(Config::Encrypt(encrypt_config)).unwrap();
        run(Config::Decrypt(decrypt_config)).unwrap();
        assert_eq!(
            fs::read("test_data/msg1.txt").unwrap(),
            fs::read("msg1.txt").unwrap()
        );
        fs::remove_file("ciphered.aes").unwrap();
        fs::remove_file("ciphered.frg").unwrap();
        fs::remove_file("msg1.txt").unwrap();
    }
}
