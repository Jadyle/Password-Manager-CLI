use crate::schema::{passwords, users};
use diesel::prelude::*;

// Define the structure to map a user record in the database
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)] // Specify the table to map the structure to
#[diesel(check_for_backend(diesel::sqlite::Sqlite))] // Specify the backend (SQLite in this case)
pub struct User {
    pub id: i32,
    pub user: String,
    pub label_account: String,
    pub master_password: String,
    pub salt_account: String,
}

// Struct for inserting a new user record into the database
#[derive(Insertable)]
#[diesel(table_name = users)] // Specify the table for insertion
pub struct NewUser<'a> {
    pub user: &'a str,
    pub label_account: &'a str,
    pub master_password: &'a str,
    pub salt_account: &'a str,
}

// Define the structure to map a password record in the database
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::passwords)] // Specify the table to map the structure to
#[diesel(check_for_backend(diesel::sqlite::Sqlite))] // Specify the backend (SQLite in this case)
pub struct Password {
    pub id: i32,
    pub label: String,
    pub r_password: String,
    pub salt_password: String,
    pub nonce_password: String,
}

// Struct for inserting a new password record into the database
#[derive(Insertable)]
#[diesel(table_name = passwords)] // Specify the table for insertion
pub struct NewPassword<'a> {
    pub label: &'a str,
    pub r_password: &'a str,
    pub salt_password: &'a str,
    pub nonce_password: &'a str,
}
