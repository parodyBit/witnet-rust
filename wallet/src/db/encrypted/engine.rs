use crate::types;

use super::*;

#[derive(Clone)]
pub struct CryptoEngine {
    key: types::Secret,
    iv: Vec<u8>,
}

impl CryptoEngine {
    pub fn new(key: types::Secret, iv: Vec<u8>) -> Self {
        Self { key, iv }
    }

    pub fn encrypt<T>(&self, value: &T) -> Result<Vec<u8>>
    where
        T: serde::Serialize + ?Sized,
    {
        let bytes = bincode::serialize(value)?;
        let encrypted = cipher::encrypt_aes_cbc(self.key.as_ref(), &bytes, &self.iv)?;

        Ok(encrypted)
    }

    pub fn decrypt<T>(&self, bytes: &[u8]) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let decrypted = cipher::decrypt_aes_cbc(self.key.as_ref(), bytes, &self.iv)?;
        let value = bincode::deserialize(&decrypted)?;

        Ok(value)
    }

    pub fn decrypt_with<T, F>(&self, bytes: &[u8], with: F) -> Result<T>
    where
        T: DeserializeOwned,
        F: Fn(&[u8]) -> Vec<u8>,
    {
        let decrypted = cipher::decrypt_aes_cbc(self.key.as_ref(), bytes, &self.iv)?;
        let with_bytes = with(&decrypted);
        let value = bincode::deserialize(&with_bytes)?;

        Ok(value)
    }
}
