use num_bigint::BigInt;
use std::str::FromStr;
use uniffi::Object;

#[derive(Debug, Clone, Object)]
pub struct Identity {
    private_key: Vec<u8>,
    secret_scalar: String,
    commitment: String,
}

#[uniffi::export]
impl Identity {
    /// Constructor exposed to UniFFI
    #[uniffi::constructor]
    pub fn new(private_key: Vec<u8>) -> Self {
        let identity = semaphore::identity::Identity::new(&private_key);
        Self {
            private_key: identity.private_key().to_vec(),
            secret_scalar: identity.secret_scalar().to_string(),
            commitment: identity.commitment().to_string(),
        }
    }

    /// Getter for private_key
    pub fn private_key(&self) -> Vec<u8> {
        self.private_key.clone()
    }

    /// Getter for commitment
    pub fn commitment(&self) -> String {
        self.commitment.clone()
    }

    /// Getter for secret_scalar
    pub fn secret_scalar(&self) -> String {
        self.secret_scalar.clone()
    }

    pub fn to_element(&self) -> Vec<u8> {
        let bytes = BigInt::from_str(&self.commitment).unwrap().to_bytes_le();
        let mut element = [0u8; 32];
        element[..bytes.1.len()].copy_from_slice(&bytes.1);
        element.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use semaphore::utils::to_element;

    use super::*;

    #[test]
    fn test_identity() {
        let identity = Identity::new("secret".as_bytes().to_vec());
        println!("{}", identity.commitment());
        println!("{:?}", identity.private_key());
        println!("{}", identity.secret_scalar());
        println!("{:?}", identity.to_element());
        let semaphore_identity = semaphore::identity::Identity::new("secret".as_bytes());
        assert_eq!(
            identity.to_element(),
            to_element(*semaphore_identity.commitment())
        );
    }
}
