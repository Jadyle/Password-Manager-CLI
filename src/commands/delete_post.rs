use diesel::prelude::*;
use password_manager::establish_connection;

use crate::commands::decrypt::is_password_decrypted;

pub fn delete_password(label_target: &str, masterpassword: &str) -> Result<(), String> {
    use password_manager::schema::passwords::dsl::*;

    // Check if label_target is empty
    if label_target.is_empty() {
        return Err(String::from(
            "\nError: No label provided. Please specify a label to delete passwords.",
        ));
    }

    let pattern = format!("%{}%", label_target);

    let connection = &mut establish_connection();

    // Check if the label exists in the database
    let label_exists = passwords
        .filter(label.like(pattern))
        .select(label)
        .first::<String>(connection)
        .optional()
        .expect("Error checking label existence");

    // Check if the password is decrypted
    if is_password_decrypted(label_target, masterpassword) {
        match label_exists {
            Some(_) => {
                // Proceed to delete the password if the label exists
                let num_delete = diesel::delete(passwords.filter(label.eq(label_target)))
                    .execute(connection)
                    .expect("Error deleting password");

                println!("\nDeleted {} password(s)", num_delete);
                Ok(())
            }
            None => {
                // If the label doesn't exist, print an error
                Err(format!(
                    "\nError: No password found with the label '{}'.",
                    label_target
                ))
            }
        }
    } else {
        // If the password is not decrypted, print an error
        Err(format!(
            "\nError. No password found with the label '{}'.",
            label_target,
        ))
    }
}
