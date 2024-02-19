pub mod config;
pub mod uuid_protocol;
pub mod crypto;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::generate_key_pair;
    use ring::signature::KeyPair;
    use crate::crypto::{verify, sign};
    
    #[test]
    fn test_sign_and_verify() {
        // Generate key pair
        let key_pair = generate_key_pair();

        // Get public key bytes
        let public_key_bytes = key_pair.public_key().as_ref();

        // Message to sign
        let message = b"Hello, world!";

        // Sign the message
        let signature = sign(&key_pair, message);

        // Verify the signature
        let is_valid = verify(public_key_bytes, message, signature.as_ref());
        
        assert!(is_valid, "Signature verification failed");
    }
}