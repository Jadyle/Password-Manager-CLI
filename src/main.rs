use std::io::{self, stdin, Write};

use commands::delete_post::delete_password;
use commands::generator::generate_password_menu;
use commands::research_post::{
    match_id_user, research_id_account, research_password, research_user_account,
    research_user_hash, research_user_salt,
};
use commands::show_posts::show_label_password;
use commands::write_post::new_password;
use commands::{show_posts::show_label_account, write_post::write_user};

mod commands;

fn main() {
    intro();
    show_label_account();
    start_password_manager();
}

fn start_password_manager() {
    let mut id_account = String::new();

    println!("\nEnter account ID ( New account => press ENTER ) :");
    entry();

    stdin().read_line(&mut id_account).unwrap(); // Read input for account ID
    let id_account = id_account.trim_end();

    // If account ID is empty, proceed to account creation
    if id_account.is_empty() {
        println!("\nLet's create a new account !");
        write_user();
    } else {
        match id_account.parse::<i32>() {
            Ok(id) => {
                match research_id_account(id) {
                    Ok(id_result) => {
                        println!("\nLOGIN TO ACCOUNT : {}", id);
                        if id_result.parse::<i32>() == Ok(id) {
                            // If account ID exists, prompt for username and password
                            connect_user(id);
                        }
                    }
                    Err(_) => {
                        eprintln!("\nError. No ID {} found.", id);
                        start_password_manager();
                    }
                }
            }
            Err(_) => {
                eprintln!("\nInvalid ID. Please enter a valid integer.");
                start_password_manager();
            }
        }
    }
}

pub fn connect_user(id: i32) {
    use crate::commands::encrypt::enc_password;

    let mut username = String::new();
    let mut masterpassword = String::new();

    print!("\nEnter your USERNAME : ");
    io::stdout().flush().unwrap();
    stdin().read_line(&mut username).unwrap(); // Read and store the username
    let username = username.trim_end();

    match match_id_user(&username, id) {
        Ok(_) => {
            print!("Enter your MASTER PASSWORD : ");
            io::stdout().flush().unwrap();
            stdin().read_line(&mut masterpassword).unwrap(); // Read and store the master password
            let masterpassword = masterpassword.trim_end();

            let salt = research_user_salt(username.to_string());
            let hash = enc_password(masterpassword, &salt);

            let username_result = research_user_account(username);
            let hash_result = research_user_hash(hash.clone());

            // Check if the provided password matches stored hash and username
            if hash == hash_result && username == username_result {
                options(masterpassword, username);
            } else {
                println!("\nFail connection");
                start_password_manager();
            }
        }
        Err(_) => {
            println!("\nError. ID {} don't match USERNAME *{}*.", id, username);
            start_password_manager();
        }
    }
}

fn intro() {
    println!(
        r#"
 ____  __   ___ ___ _    _ _____ ____ ____     __  __   __   _  _   __   ___ ____ ____ 
(  _ \/__\ / __/ __( \/\/ (  _  (  _ (  _ \   (  \/  ) /__\ ( \( ) /__\ / __( ___(  _ \
 )___/(__)\\__ \__ \)    ( )(_)( )   /)(_) )   )    ( /(__)\ )  ( /(__)( (_-.)__) )   /
(__)(__)(__(___(___(__/\__(_____(_)\_(____/   (_/\/\_(__)(__(_)\_(__)(__\___(____(_)\_)

Created by Lina Rashdan.

Ps: This is an educational project. Please **do not** use it for storing real passwords.

**Important:**
- This is an **educational version** designed to demonstrate all the steps involved in **encryption** and **decryption** during function calls.
- All keys, hashes, and other cryptographic details are displayed during function calls to explain how encryption and decryption work.
- The password manager is for educational purposes only and should not be used for sensitive data.
- **Use it at your own risk**
"#
    );
}

pub fn options(masterpassword: &str, username: &str) {
    loop {
        clear_terminal();
        intro();
        println!(
            r#"
------------------------------

 * Connected user : {username} 

------------------------------

Choose your option :
    Enter new password => c
    Generate a password => g
    Research a password => s
    View all password label => v
    Delete a password => d
    Quit => q"#
        );

        entry();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim_end();

        match input {
            "c" => {
                new_password(masterpassword, None, None); // Create a new password
                if !return_to_options() {
                    break;
                }
            }
            "s" => {
                println!("\nEnter the label of the password : ");
                entry();
                let mut label = String::new();
                io::stdin().read_line(&mut label).unwrap();
                let label = label.trim_end();
                let _ = research_password(label, masterpassword); // Search for a password by label

                if !return_to_options() {
                    break;
                }
            }
            "v" => {
                show_label_password(masterpassword, username); // Show all password labels
                if !return_to_options() {
                    break;
                }
            }
            "g" => {
                let (label, password) = generate_password_menu(); // Generate a new password

                print!("\nSave the password ? (y/n) : ");
                io::stdout().flush().unwrap();
                let mut save = String::new();
                io::stdin().read_line(&mut save).unwrap();

                // Option to save the generated password
                if save.trim().eq_ignore_ascii_case("y") {
                    new_password(masterpassword, Some(&label), Some(&password));
                } else {
                    println!("\nPassword not saved.")
                }
                if !return_to_options() {
                    break;
                }
            }
            "d" => {
                println!("\nEnter the label of the password you want to delete : ");
                entry();
                let mut label = String::new();
                io::stdin().read_line(&mut label).unwrap();
                let label = label.trim_end();
                match delete_password(label, masterpassword) {
                    Ok(_) => {
                        println!("Password deleted successfully !");
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                }

                if !return_to_options() {
                    break;
                }
            }
            "q" => {
                println!("\nExiting the program. Goodbye!");
                break;
            }
            _ => {
                println!("Invalid option, please try again.");
            }
        }
    }
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[H"); // Clear the terminal screen
    io::stdout().flush().unwrap();
}

fn return_to_options() -> bool {
    println!("\n\nPress ENTER to return to the options menu.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    match input.trim_end() {
        _ => {
            // Invalid input. Returning to options by default.
            true
        }
    }
}

pub fn entry() {
    print!("> ");
    io::stdout().flush().unwrap();
}
