use diesel::prelude::*;
use password_manager::{establish_connection, models::*};

use crate::commands::decrypt::is_password_decrypted;

// Function to show all label accounts from the users table
pub fn show_label_account() {
    use password_manager::schema::users::dsl::*;

    // Establish connection to the database
    let connection = &mut establish_connection();

    println!(
        r#"
Let's start !

Here is all labels accounts :"#
    );

    let results = users.select(User::as_select()).load::<User>(connection);

    match results {
        Ok(user_d) => {
            println!("\nLoaded {} users.\n", user_d.len());
            for master_user in user_d {
                println!("({}) {}", master_user.id, master_user.label_account);
            }
        }
        Err(e) => {
            eprintln!("Error loading users: {}", e);
        }
    }
}

// Function to show all labels associated with passwords for a specific user
pub fn show_label_password(masterpassword: &str, user_pass: &str) {
    use password_manager::schema::passwords::dsl::*;

    // Establish connection to the database
    let connection = &mut establish_connection();

    // Query all password records and load the results into 'results'
    let results = passwords
        .select(Password::as_select()) // Select all columns from the Password model
        .load(connection) // Execute the query and load the results
        .expect("Connection error"); // Handle any connection error

    // Create a vector to store labels of decrypted passwords
    let mut vec_label: Vec<String> = Vec::new();

    // Iterate through each password record
    for label_pass in results {
        let label_str = label_pass.label.clone();

        // Check if the password is decrypted by calling the 'is_password_decrypted' function
        if is_password_decrypted(&label_str, masterpassword) {
            vec_label.push(label_str); // Add the label to the vector if decrypted
        }
    }

    // Check if any labels were found and print the appropriate message
    if vec_label.is_empty() {
        // If no labels are found, print a message indicating no passwords found for the user
        println!("\nNo passwords found for the user: * {} *", user_pass);
    } else {
        // If labels are found, print them
        println!(
            "\nPassword labels registered for the user * {} * : ",
            user_pass
        );
        for label_pass in vec_label {
            println!("> {}", label_pass); // Print each label found
        }
    }
}
