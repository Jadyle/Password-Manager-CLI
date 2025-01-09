use password_manager::{add_password, establish_connection, models::Password};
use password_manager::{create_user, models::User};
use std::io::{self, stdin, Write};

use crate::commands::encrypt::{generate_key, hash_aes256_password}; // For generating encryption key and hashing the password

pub fn write_user() {
    use crate::commands::encrypt::{enc_password, generate_salt};

    // Establish a connection to the database
    let connection = &mut establish_connection();

    // Variables for storing user input
    let mut owner = String::new();
    let mut masterpassword = String::new();
    let mut labelaccount = String::new();

    // Prompt user for account label (e.g., username or account type)
    print!("\nEnter a label for the account : ");
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed
    stdin().read_line(&mut labelaccount).unwrap(); // Read user input for the label
    let labelaccount = labelaccount.trim_end(); // Remove trailing newlines

    // Check if the label is valid (not empty)
    if labelaccount.is_empty() || labelaccount == "\n" {
        eprintln!("\nError: No label provided. Please enter a valid label.");
        return; // Exit if label is empty
    }

    // Prompt user for username
    print!("Enter the user name : ");
    io::stdout().flush().unwrap();
    stdin().read_line(&mut owner).unwrap(); // Read user input for username
    let owners = owner.trim_end(); // Clean the input by removing whitespace

    // Check if the username is valid (not empty)
    if owner == "\n" || owner.is_empty() {
        eprintln!("\nError: No user name provided. Please enter a valid user name.");
        return; // Exit if username is empty
    }

    // Prompt user for master password
    print!("Enter the master password : ");
    io::stdout().flush().unwrap();
    stdin().read_line(&mut masterpassword).unwrap(); // Read user input for master password
    let masterpassword = masterpassword.trim_end(); // Remove trailing newlines

    // Check if the master password is valid (not empty)
    if masterpassword.is_empty() || masterpassword == "\n" {
        eprintln!("\nError: No master password provided. Please enter a valid master password.");
        return; // Exit if master password is empty
    }

    // Generate a salt for password hashing
    let mp_salt = generate_salt();

    // Encrypt and hash the master password using the generated salt
    let masterpassword_hash = enc_password(masterpassword, &mp_salt);

    // Create a new user in the database with the provided information
    let users: User = create_user(
        connection,
        owners,
        labelaccount,
        &masterpassword_hash,
        &mp_salt,
    ); // Store the user information in the database

    // Output the result to the user
    println!(
        "\nSaved user {} with hash {}",
        users.user, users.master_password
    );
}

// Function to add a new password to the database
pub fn new_password(masterpassword: &str, password_label: Option<&str>, password: Option<&str>) {
    // Establish a connection to the database
    let connection = &mut establish_connection();

    // Get the password label either from user input or from the function argument
    let password_label = match password_label {
        Some(label) => label.to_string(), // If the label is provided, use it
        None => {
            let mut input = String::new();
            print!("\nEnter the label : ");
            io::stdout().flush().unwrap(); // Ensure the prompt is displayed
            io::stdin().read_line(&mut input).unwrap(); // Read the user input
            let label = input.trim().to_string();
            if label.is_empty() {
                // Ensure the label is not empty
                eprintln!("\nError. Please enter a label.");
                return; // Exit the function if the label is empty
            }
            label // Return the label entered by the user
        }
    };

    // Get the password either from user input or from the function argument
    let password = match password {
        Some(pwd) => pwd.to_string(), // If the password is provided, use it
        None => {
            let mut input = String::new();
            print!("Enter the password : ");
            io::stdout().flush().unwrap(); // Ensure the prompt is displayed
            io::stdin().read_line(&mut input).unwrap(); // Read the user input
            let password = input.trim_end().to_string();
            if password.is_empty() {
                // Ensure the password is not empty
                eprintln!("\nError. Please enter a password.");
                return; // Exit the function if the password is empty
            }
            password // Return the password entered by the user
        }
    };

    let bytes_password = password.as_bytes(); // Convert the password to bytes

    // Generate the encryption key and salt using the master password
    let (key, salt_password) = generate_key(masterpassword);

    // Hash the password using AES-256 encryption
    let (hash, nonce) = hash_aes256_password(bytes_password, key).unwrap();

    // Add the hashed password to the database
    let password: Password =
        add_password(connection, &password_label, &hash, &salt_password, &nonce); // Store the password in the database

    // Output the result to the user
    println!(
        "\nPassword saved ! \n\nLABEL : {} \nID : {}",
        password.label, password.id
    );
}
