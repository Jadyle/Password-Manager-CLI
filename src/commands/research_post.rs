use diesel::prelude::*;
use password_manager::{
    establish_connection,
    models::User,
    schema::{
        passwords::{label, nonce_password, r_password, salt_password},
        users::{id, master_password, salt_account, user},
    },
};

use crate::commands::decrypt::dec_aes_password;

// Search for a user by account ID
pub fn research_id_account(id_account: i32) -> Result<String, String> {
    use password_manager::schema::users::dsl::users;

    let connection = &mut establish_connection();

    match users
        .filter(id.eq(&id_account))
        .select(User::as_select())
        .first(connection)
        .optional()
    {
        Ok(Some(master_user)) => Ok(master_user.id.to_string()),
        Ok(None) => Err(format!("No ID found.")),
        Err(_) => Err(format!("No ID found.")),
    }
}

// Match a username with a user ID
pub fn match_id_user(username: &str, id_match: i32) -> Result<String, String> {
    use password_manager::schema::users::dsl::users;

    let connection = &mut establish_connection();

    match users
        .filter(user.eq(&username))
        .select(id)
        .first::<i32>(connection)
        .optional()
    {
        Ok(Some(user_id)) => {
            if id_match == user_id {
                Ok(user_id.to_string())
            } else {
                Err(format!("ID do not match user name."))
            }
        }

        Ok(None) => Err(format!(
            "No match found for username '{}' and ID '{}'.",
            username, id_match
        )),
        Err(_) => Err(format!("Error querying the database.")),
    }
}

// Search for a user by their username
pub fn research_user_account(user_name: &str) -> String {
    use password_manager::schema::users::dsl::users;

    let connection = &mut establish_connection();

    match users
        .filter(user.eq(&user_name))
        .select(User::as_select())
        .first(connection)
        .optional()
    {
        Ok(Some(master_user)) => master_user.user,
        Ok(None) => String::from("Incorrect value"),
        Err(_) => String::from("Incorrect value"),
    }
}

// Search for a user by their master password hash
pub fn research_user_hash(hash: String) -> String {
    use password_manager::schema::users::dsl::users;

    let connection = &mut establish_connection();

    match users
        .filter(master_password.eq(&hash))
        .select(User::as_select())
        .first(connection)
        .optional()
    {
        Ok(Some(master_user)) => master_user.master_password,
        Ok(None) => String::from("Incorrect value"),
        Err(_) => String::from("Incorrect value"),
    }
}

// Search for a user's salt by their username
pub fn research_user_salt(username: String) -> String {
    use password_manager::schema::users::dsl::users;

    let connection = &mut establish_connection();

    match users
        .filter(user.eq(&username))
        .select(salt_account)
        .first::<String>(connection)
        .optional()
    {
        Ok(Some(salt)) => salt,
        Ok(None) => String::from("Incorrect value"),
        Err(_) => String::from("Incorrect value"),
    }
}

// Search for a password salt by its label
pub fn research_password_salt(label_pass: String) -> String {
    use password_manager::schema::passwords::dsl::passwords;

    let connection = &mut establish_connection();

    match passwords
        .filter(label.eq(&label_pass))
        .select(salt_password)
        .first::<String>(connection)
        .optional()
    {
        Ok(Some(salt)) => salt,
        Ok(None) => String::from("Incorrect value"),
        Err(_) => String::from("Incorrect value"),
    }
}

// Search for the password associated with a label, given a master password
pub fn research_password(labels: &str, masterpassword: &str) -> Result<String, aes_gcm::Error> {
    use password_manager::schema::passwords::dsl::passwords;

    let connection = &mut establish_connection();

    if labels.is_empty() {
        eprintln!("\nError. Please enter a label.");
        return Err(aes_gcm::Error);
    }

    match passwords
        .filter(label.eq(&labels))
        .select((r_password, nonce_password, label))
        .first::<(String, String, String)>(connection)
        .optional()
    {
        Ok(Some((hash, nonce, label_pass))) => {
            match dec_aes_password(&hash, &nonce, masterpassword, &label_pass) {
                Ok(password) => {
                    println!("\n LABEL    : {label_pass} \n PASSWORD : {password}");
                    Ok(password)
                }
                Err(e) => {
                    println!("\n{:?}. Make sure to be the owner of the password.", e);
                    Err(aes_gcm::Error)
                }
            }
        }
        Ok(None) => {
            if labels.is_empty() {
                eprintln!("\nError. Please enter a label.");
                Err(aes_gcm::Error)
            } else {
                eprintln!("\nNo password found under the label: \"{}\".", labels);
                Err(aes_gcm::Error)
            }
        }
        Err(_) => Err(aes_gcm::Error),
    }
}

// Search for a password by its label and master password
pub fn research_label(labels: &str, masterpassword: &str) -> Result<String, aes_gcm::Error> {
    use password_manager::schema::passwords::dsl::passwords;

    let connection = &mut establish_connection();

    match passwords
        .filter(label.eq(&labels))
        .select((r_password, nonce_password, label))
        .first::<(String, String, String)>(connection)
        .optional()
    {
        Ok(Some((hash, nonce, label_pass))) => {
            match dec_aes_password(&hash, &nonce, masterpassword, &label_pass) {
                Ok(password) => Ok(password),
                Err(e) => {
                    println!("\n{:?}. You're not the owner of the password.", e);
                    Err(aes_gcm::Error)
                }
            }
        }
        Ok(None) => Err(aes_gcm::Error),
        Err(_) => Err(aes_gcm::Error),
    }
}
