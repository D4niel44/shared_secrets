pub mod shamir;

use std::error::Error;
use std::fmt;

use aes_gcm::aead::{generic_array::GenericArray, Aead, NewAead};
use aes_gcm::Aes256Gcm;

use sha2::{Digest, Sha256};

pub use crate::crypto::shamir::{Share, ShareIter};

/// A AES-256 cipher which supports splitting keys
/// into shares using shamir secret sharing scheme
pub struct Cipher {
    aes: Aes256Gcm,
    key: Vec<u8>,
}

impl Cipher {
    /// Creates a new cipher using the string given
    /// to generate the key.
    ///
    /// # Parameters
    ///
    /// - key: A string given by the user, used for
    /// generating a key.
    ///
    /// # Returns
    ///
    /// A cipher with a given generated key.
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
    /// # Parameters
    ///
    /// - plaintext: An array of bytes that represent
    /// a given decrypted block.
    ///
    /// # Returns
    ///
    /// A vector of encrypted bytes.
    ///
    /// # Errors
    ///
    /// This method returns an error if an error occurs while encrypting
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, CipherError> {
        match self
            .aes
            .encrypt(GenericArray::from_slice(&[0x44u8; 12]), plaintext)
        {
            Ok(v) => Ok(v),
            Err(_) => Err(CipherError("Error while encrypting".into())),
        }
    }

    /// Decrypts the given block in place.
    ///
    /// # Parameters
    ///
    /// - ciphertext: An array of bytes that represent
    /// a given encrypted block.
    ///
    /// # Returns
    ///
    /// A vector of decrypted bytes.
    ///
    /// # Errors
    ///
    /// This method returns an error if an error occurs while decrypting
    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, CipherError> {
        match self
            .aes
            .decrypt(GenericArray::from_slice(&[0x44u8; 12]), ciphertext)
        {
            Ok(v) => Ok(v),
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
    ///
    /// A ShareIter with n Shares.
    ///
    /// # Panics
    ///
    /// This method panics if the parameters constraints are not met.
    pub fn split_key(&self, n: usize, k: usize) -> ShareIter {
        shamir::split_secret(&self.key, n, k)
    }
}

/// An error that may occur when encrypting or decrypting.
#[derive(Debug, Clone)]
pub struct CipherError(pub String);

impl fmt::Display for CipherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for CipherError {}

/////////////////////////////////
// Unit testing of the module. //
/////////////////////////////////

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
        let message = b"This is a message";
        let ciphertext = cipher.encrypt(message.as_ref()).unwrap();
        let result = cipher.decrypt(ciphertext.as_ref()).unwrap();
        assert_eq!(&result, b"This is a message");
    }

    #[test]
    fn integrity_new() {
        let cipher = Cipher::new("This is a secure key");
        let message = b"This is a message";
        let ciphertext = cipher.encrypt(message.as_ref()).unwrap();
        let result = cipher.decrypt(ciphertext.as_ref()).unwrap();
        assert_eq!(&result, b"This is a message");
    }

    #[test]
    fn integrity_from_shares() {
        let key = [0x12u8; 32];
        let cipher = Cipher {
            aes: Aes256Gcm::new(GenericArray::from_slice(&key)),
            key: key.to_vec(),
        };
        let ciphertext = cipher.encrypt(b"message".as_ref()).unwrap();
        let shares = cipher.split_key(4, 3);
        let decipher = Cipher::from_shares(shares).unwrap();
        let plaintext = decipher.decrypt(ciphertext.as_ref()).unwrap();
        assert_eq!(&plaintext, b"message");
    }
}
