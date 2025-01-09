use aes::cipher::{generic_array::GenericArray, typenum};
use aes_gcm::{aead::Aead, AeadCore, Aes256Gcm, KeyInit};
use base64::engine::{general_purpose, Engine};
use pbkdf2::pbkdf2_hmac;
use rand::{rngs::OsRng, Rng};
use sha2::{Digest, Sha256};

// Encrypt the password with salt
pub fn enc_password(password: &str, salt: &str) -> String {
    println!("Starting the encryption of the password with salt...");
    let password = password;
    let hashed_password = hash_password(password, &salt);
    println!("Encrypted password (hashed with salt): {}", hashed_password);
    hashed_password
}

// Generate a random salt for the password
pub fn generate_salt() -> String {
    println!("\nGenerating a random salt for the password...");
    let mut random = rand::thread_rng();
    let salt: [u8; 16] = random.gen();
    let encoded_salt = general_purpose::STANDARD.encode(salt);
    println!("Generated Salt (base64): {}", encoded_salt);
    encoded_salt
}

// Hash the password with the salt using SHA256
pub fn hash_password(password: &str, salt: &str) -> String {
    println!("Hashing the password with salt...");

    let salted_password = format!("{}{}", password, salt);
    println!("Salted password: {}", salted_password);

    let mut init_hash = Sha256::new();
    init_hash.update(salted_password);
    let hash = init_hash.finalize();
    let encoded_hash = general_purpose::STANDARD.encode(hash);

    println!("Hashed password (SHA256): {}", encoded_hash);
    encoded_hash
}

// Encrypt the password with AES-256-GCM
pub fn hash_aes256_password(
    password: &[u8],
    key: GenericArray<u8, typenum::U32>,
) -> Result<(String, String), aes_gcm::Error> {
    println!("Encrypting the password using AES-256-GCM...");

    let aes_key = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    println!(
        "Generated nonce: {}",
        general_purpose::STANDARD.encode(&nonce)
    );

    let hash = aes_key.encrypt(&nonce, password)?;
    println!(
        "Encrypted password (AES256): {}",
        general_purpose::STANDARD.encode(&hash)
    );

    let encoded_hash = general_purpose::STANDARD.encode(&hash);
    let encoded_nonce = general_purpose::STANDARD.encode(&nonce);

    Ok((encoded_hash, encoded_nonce))
}

// Generate a new random salt for AES
pub fn generate_new_salt() -> [u8; 16] {
    println!("Generating a new random salt...");
    let mut random = rand::thread_rng();
    let salt = random.gen();
    println!("Generated new salt: {:?}", salt);
    salt
}

// Generate an encryption key from the master password using PBKDF2
pub fn generate_key(masterpassword: &str) -> (GenericArray<u8, typenum::U32>, String) {
    println!("\nGenerating encryption key from master password using PBKDF2...");

    let masterpassword = masterpassword.as_bytes();
    // Store the salt
    let salt = generate_new_salt();
    println!("Salt used in PBKDF2: {:?}", salt);

    let n = 100_000;
    let mut key = GenericArray::<u8, typenum::U32>::default();

    pbkdf2_hmac::<Sha256>(masterpassword, &salt, n, &mut key);
    println!("Generated encryption key (from PBKDF2): {:?}", key);

    let encoded_salt = general_purpose::STANDARD.encode(&salt);
    println!("Encoded salt (base64): {}", encoded_salt);

    (key, encoded_salt)
}
