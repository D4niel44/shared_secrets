use std::error::Error;

use aes_gcm::aead::{generic_array::GenericArray, AeadInPlace, NewAead};
use aes_gcm::Aes256Gcm;
use sha2::{Digest, Sha256};

use error::CipherError;

pub use crate::crypto::shamir::{Share, ShareIter};

pub mod error;
pub mod shamir;

/// A AES-256 cipher which supports splitting keys
/// into shares using shamir secret sharing scheme
pub struct Cipher {
    aes: Aes256Gcm,
    key: Vec<u8>,
}

impl Cipher {
    /// Creates a new cipher using the string given
    /// to generate the key.
    pub fn new(key: &str) -> Self {
        let key = Sha256::digest(key.as_bytes());
        Cipher {
            aes: Aes256Gcm::new(&key),
            key: key.iter().cloned().collect(),
        }
    }

    /// Creates an cipher from an iterator of shares.
    ///
    /// # Parameters
    ///
    /// - shares: An Iterator of shares, require that each share has a
    /// unique first element.
    ///
    /// # Returns
    ///
    /// A cipher that uses the recovered secret from the Shares as key.
    ///
    /// # Errors
    ///
    /// This method returns an error if there are two shares with the same
    /// first element or if it parse the shares.
    pub fn from_shares(shares: impl Iterator<Item = Share>) -> Result<Self, Box<dyn Error>> {
        let key = shamir::recover_secret(shares)?;
        if key.len() != 32 {
            return Err(Box::new(CipherError(
                "Error while recovering key from shares".into(),
            )));
        }
        Ok(Cipher {
            aes: Aes256Gcm::new(GenericArray::from_slice(&key)),
            key,
        })
    }

    /// Encrypts the given block in place.
    ///
    /// # Errors
    ///
    /// This method returns an error if an error occurs while encrypting
    pub fn encrypt(&self, plaintext: &mut Vec<u8>) -> Result<(), CipherError> {
        match self
            .aes
            .encrypt_in_place(GenericArray::from_slice(&[0x44u8; 12]), b"", plaintext)
        {
            Ok(_) => Ok(()),
            Err(_) => Err(CipherError("Error while encrypting".into())),
        }
    }

    /// Decrypts the given block in place.
    ///
    /// # Errors
    ///
    /// This method returns an error if an error occurs while encrypting
    pub fn decrypt(&self, ciphertext: &mut Vec<u8>) -> Result<(), CipherError> {
        match self
            .aes
            .decrypt_in_place(GenericArray::from_slice(&[0x44u8; 12]), b"", ciphertext)
        {
            Ok(_) => Ok(()),
            Err(_) => Err(CipherError("Error while decrypting".into())),
        }
    }

    /// Splits the key of this cipher into n shares with
    /// only needing k to recover the original key.
    ///
    /// # Parameter
    ///
    /// - n: The total number of shares to return (n > 2)
    /// - k: The minimum number of shares to recover the secret (0 < k <= n)
    ///
    /// # Returns
    /// A ShareIter with n Shares.
    ///
    /// # Panics
    ///
    /// This method panics if the parameters constraints are not met.
    pub fn split_key(&self, n: usize, k: usize) -> ShareIter {
        shamir::split_secret(&self.key, n, k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integrity() {
        let key = [0x10u8; 32];
        let cipher = Cipher {
            aes: Aes256Gcm::new(GenericArray::from_slice(&key)),
            key: key.to_vec(),
        };
        let mut message = b"This is a message".to_vec();
        cipher.encrypt(&mut message).unwrap();
        cipher.decrypt(&mut message).unwrap();
        assert_eq!(&message, b"This is a message");
    }

    #[test]
    fn integrity_new() {
        let cipher = Cipher::new("This is a secure key");
        let mut message = b"This is a message".to_vec();
        cipher.encrypt(&mut message).unwrap();
        cipher.decrypt(&mut message).unwrap();
        assert_eq!(&message, b"This is a message");
    }

    #[test]
    fn integrity_from_shares() {
        let key = [0x12u8; 32];
        let cipher = Cipher {
            aes: Aes256Gcm::new(GenericArray::from_slice(&key)),
            key: key.to_vec(),
        };
        let mut message = b"message".to_vec();
        cipher.encrypt(&mut message).unwrap();
        let shares = cipher.split_key(4, 3);
        let decipher = Cipher::from_shares(shares).unwrap();
        decipher.decrypt(&mut message).unwrap();
        assert_eq!(&message, b"message");
    }
}
