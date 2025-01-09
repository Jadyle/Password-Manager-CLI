use aes::cipher::{
    consts::{B0, B1},
    generic_array::GenericArray,
    typenum::{self, UInt, UTerm},
};
use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit, Nonce};
use base64::{engine::general_purpose, Engine};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

use crate::commands::research_post::research_password_salt;

use super::research_post::research_label;

pub fn dec_aes_password(
    hash: &str,
    nonce: &str,
    masterpassword: &str,
    label: &str,
) -> Result<String, aes_gcm::Error> {
    println!("\nDecrypting password...");

    // Decode the encrypted password hash (base64)
    println!("Decoding the encrypted password hash (base64)...");
    let decode_hash = general_purpose::STANDARD
        .decode(&hash)
        .map_err(|_| aes_gcm::Error)?;
    println!("Decoded encrypted password hash: {:?}", decode_hash);

    // Decode the nonce (base64)
    println!("Decoding the nonce (base64)...");
    let decode_nonce = general_purpose::STANDARD
        .decode(&nonce)
        .map_err(|_| aes_gcm::Error)?;
    println!("Decoded nonce: {:?}", decode_nonce);

    // Fetch the salt based on the label
    println!("Fetching the salt based on the label...");
    let salt = research_password_salt(label.to_string());
    println!("Salt retrieved for password: {}", salt);

    // Generate the AES key from the master password
    println!("Generating the AES key from the master password...");
    let key = get_key(masterpassword, &salt).unwrap();
    println!("Generated AES key: {:?}", key);

    // Create the AES cipher with the generated key
    let aes_key = Aes256Gcm::new(&key);

    // Convert the nonce to a slice for AES
    let nonce = Nonce::from_slice(&decode_nonce);
    println!("Nonce converted to slice for AES: {:?}", nonce);

    // Decrypt the password with AES256-GCM
    println!("Decrypting the password with AES256-GCM...");
    let decrypt_password = aes_key.decrypt(&nonce, decode_hash.as_ref()).map_err(|_| {
        println!("Error during decryption."); // Show error if decryption fails
        aes_gcm::Error
    })?;
    println!("Decrypted password !",);

    // Return the decrypted password as a string
    Ok(String::from_utf8_lossy(&decrypt_password).to_string())
}

pub fn get_key(
    masterpassword: &str,
    salt: &str,
) -> Result<
    GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>>,
    aes_gcm::Error,
> {
    println!("Generating AES key using PBKDF2 with the master password and salt...");

    let masterpassword = masterpassword.as_bytes();

    // Decode the salt (base64)
    println!("Decoding the salt (base64)...");
    let decode_salt = general_purpose::STANDARD.decode(&salt).map_err(|_| {
        println!("Error decoding the salt: {}", salt); // Show error if decoding salt fails
        aes_gcm::Error
    })?;
    println!("Decoded salt: {:?}", decode_salt);

    let n = 100_000;

    // Create an array for the key (initialized by default)
    let mut key = GenericArray::<u8, typenum::U32>::default();

    // Apply PBKDF2 to derive the AES key
    println!("Applying PBKDF2 to derive the AES key...");
    pbkdf2_hmac::<Sha256>(masterpassword, &decode_salt, n, &mut key);
    println!("Generated key from PBKDF2: {:?}", key);

    Ok(key)
}

pub fn is_password_decrypted(label: &str, masterpassword: &str) -> bool {
    match research_label(label, masterpassword) {
        Ok(_) => true,
        Err(_) => false,
    }
}
