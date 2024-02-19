use ring::rand::{SecureRandom, SystemRandom};
use ring::signature::{Ed25519KeyPair, KeyPair, Signature, UnparsedPublicKey};
use std::error::Error;
use std::fs::File;
use std::io::Read;

pub fn generate_key_pair() -> Ed25519KeyPair {
    let rng = SystemRandom::new();
    let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();
    key_pair
}

pub fn sign(key_pair: &Ed25519KeyPair, message: &[u8]) -> Vec<u8> {
    let signature = key_pair.sign(message);
    signature.as_ref().to_vec()
}

pub fn verify(public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    let public_key = UnparsedPublicKey::new(&ring::signature::ED25519, public_key);
    public_key.verify(message, signature).is_ok()
}

pub fn read_key_pair<P>(private_key_path: P, public_key_path: P) -> Result<Ed25519KeyPair, Box<dyn Error>>
where
    P: AsRef<std::path::Path>,
{
    let mut private_key_file = File::open(private_key_path)?;
    let mut private_key_bytes = Vec::new();
    private_key_file.read_to_end(&mut private_key_bytes)?;

    let mut public_key_file = File::open(public_key_path)?;
    let mut public_key_bytes = Vec::new();
    public_key_file.read_to_end(&mut public_key_bytes)?;

    let key_pair = Ed25519KeyPair::from_pkcs8(private_key_bytes.as_ref()).unwrap();
    let public_key = key_pair.public_key().as_ref();

    // Ensure that the provided public key matches the one in the private key file
    if public_key != public_key_bytes.as_slice() {
        return Err("Public / Private key mismatch".into());
    }

    Ok(key_pair)
}

fn read_public_key<P>(public_key_path: P) -> Result<Vec<u8>, Box<dyn Error>>
where
    P: AsRef<std::path::Path>,
{
    let mut public_key_file = File::open(public_key_path)?;
    let mut public_key_bytes = Vec::new();
    public_key_file.read_to_end(&mut public_key_bytes)?;
    Ok(public_key_bytes)
}