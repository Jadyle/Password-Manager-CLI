use rand::Rng;
use std::io::{self, Write};

/// Generates a password based on user choices.

/// A randomly generated password.
pub fn generate_password(
    length: usize,
    use_alpha: bool,
    use_numeric: bool,
    use_special: bool,
) -> String {
    let alpha: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
    let numeric: Vec<char> = ('0'..='9').collect();
    let special: Vec<char> = "!@#$%^&*()-_=+[]{}|;:',.<>?/".chars().collect();

    // Build the list of possible characters based on user choices
    let mut charset: Vec<char> = Vec::new();
    if use_alpha {
        charset.extend(&alpha);
    }
    if use_numeric {
        charset.extend(&numeric);
    }
    if use_special {
        charset.extend(&special);
    }

    if charset.is_empty() {
        panic!("Need to select at least one character type!");
    }

    let mut rng = rand::thread_rng();

    // Generate the password
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx]
        })
        .collect()
}

/// Prompts the user for password options and generates a password.
pub fn generate_password_menu() -> (String, String) {
    println!("\nLet's generate a password!");

    print!("\nEnter a label: ");
    io::stdout().flush().unwrap();
    let mut label = String::new();
    io::stdin().read_line(&mut label).unwrap();
    let label = label.trim().to_string();

    if label.is_empty() {
        eprintln!("\nError. A label is required.");
        generate_password_menu();
        return (Default::default(), Default::default());
    }

    // Ask for password length
    print!("Enter the length of the password: ");
    io::stdout().flush().unwrap();
    let mut length_input = String::new();
    io::stdin().read_line(&mut length_input).unwrap();
    let length: usize = match length_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("\nError. Enter a valid number");
            generate_password_menu();
            return (Default::default(), Default::default());
        }
    };

    // Ask for character types
    print!("Do you want to include alphabetic characters? (y/n): ");
    io::stdout().flush().unwrap();
    let mut alpha_input = String::new();
    io::stdin().read_line(&mut alpha_input).unwrap();
    let use_alpha = alpha_input.trim().eq_ignore_ascii_case("y");

    print!("Do you want to include numeric characters? (y/n): ");
    io::stdout().flush().unwrap();
    let mut numeric_input = String::new();
    io::stdin().read_line(&mut numeric_input).unwrap();
    let use_numeric = numeric_input.trim().eq_ignore_ascii_case("y");

    print!("Do you want to include special characters? (y/n): ");
    io::stdout().flush().unwrap();
    let mut special_input = String::new();
    io::stdin().read_line(&mut special_input).unwrap();
    let use_special = special_input.trim().eq_ignore_ascii_case("y");

    // Check that at least one character type is selected
    if !use_alpha && !use_numeric && !use_special {
        eprintln!("\nError. Select at least one type of character!");
        generate_password_menu();
        return (Default::default(), Default::default());
    }

    // Generate the password
    let password = generate_password(length, use_alpha, use_numeric, use_special);

    println!("\nGenerated password: {}", password);
    (label, password)
}
