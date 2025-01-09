use self::models::{NewUser, User};
use diesel::connection::SimpleConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use models::{NewPassword, Password};
use std::env;

//Declare module
pub mod models;
pub mod schema;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

//Establish the connection with SQLite
pub fn establish_connection() -> SqliteConnection {
    //Declare a PATH that is in .env file
    //'dotenv' crate search the PATH in the .env file
    //'ok' is made to continue the program even if there is no PATH in .env
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let database_password =
        env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set in .env");

    //Establish the connection to the database
    //If the connection don't work, it retourn a panic
    let mut conn = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    // Apply key
    conn.batch_execute(&format!("PRAGMA key='{}';", database_password))
        .unwrap();

    // Configure SQLite
    conn.batch_execute(
        "
        PRAGMA busy_timeout = 1000;
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;
        PRAGMA foreign_keys = ON;
    ",
    )
    .unwrap();

    // Apply migration
    conn.run_pending_migrations(MIGRATIONS).unwrap();

    conn
}
// Function to save a new post
// conn is used to communicate with the DB
// Function return a Post type (defined in models.rs)
pub fn create_user(
    conn: &mut SqliteConnection,
    user: &str,
    label_account: &str,
    master_password: &str,
    salt_account: &str,
) -> User {
    use crate::schema::users;
    // Create a new object defined as the NewPost struct
    let new_user = NewUser {
        user,
        label_account,
        master_password,
        salt_account,
    };
    // Insertion in the DB
    diesel::insert_into(users::table) // Specify the table to insert to
        .values(&new_user) // Indicate that the new value is new_post
        .returning(User::as_returning()) // Return the result like a Post object
        .get_result(conn) // Execute the insertion and return the result with the connexion (conn)
        .expect("Error saving new user")
}

pub fn add_password(
    conn: &mut SqliteConnection,
    label: &str,
    r_password: &str,
    salt_password: &str,
    nonce_password: &str,
) -> Password {
    use crate::schema::passwords;
    // Create a new object defined as the NewPost struct
    let new_password = NewPassword {
        label,
        r_password,
        salt_password,
        nonce_password,
    };
    // Insertion in the DB
    diesel::insert_into(passwords::table) // Specify the table to insert to
        .values(&new_password) // Indicate that the new value is new_post
        .returning(Password::as_returning()) // Return the result like a Post object
        .get_result(conn) // Execute the insertion and return the result with the connexion (conn)
        .expect("Error saving new user")
}
