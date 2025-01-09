# Password-manager

### ⚠️ **Disclaimer: Educational Use Only** ⚠️

This project is designed **exclusively for educational purposes** and aims to demonstrate the underlying processes of **encryption** and **decryption**. It is **not intended for securing sensitive or confidential data**. Please review the following points carefully:  

- **Do not store real passwords**: This project is not secure for managing actual sensitive information.  
- **Transparency for learning**: Keys, hashes, and cryptographic details are displayed during function execution to help users understand how encryption and decryption work.  
- **Limited security**: The focus is on education, not on creating a production-level password manager.  
- **Use at your own risk**: The authors are not responsible for any misuse or data compromise resulting from this project.  

By using this project, you acknowledge and accept these limitations.

## Authors
- Rashdan, Lina

## Compatibility
- Rust - 1.82.0

## Overview

This project is a simple Command-Line Interface (CLI) application designed to show all the involved proccess,encryption and decryption, for educationnal purpose. it should not be use for storing confidential data.

```bash
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
```

## Features

The password manager offers a set of functionalities designed for managing user accounts and passwords in an educational setting. Here's an overview of the key features available in the program:

1. **Account Management:**
   - Users can log in using an existing account ID, or create a new account if no ID is entered.
   - If the account ID exists, the user can log in by entering their username and master password.
   - If the account ID doesn't exist, the program prompts the user to re-enter a valid account ID.

2. **Password Management Options:**
   Once logged in, users can choose from the following options:
   
   - **Create a New Password (`c`):**  
     Users can create a new password, which will be encrypted and stored for future access.

   - **Generate a Password (`g`):**  
     A secure, random password is generated for the user. The generated password can be saved or discarded based on the user's preference.

   - **Search for a Password (`s`):**  
     Users can search for a stored password by entering its label. The program will decrypt the password and display it in clear text, provided the decryption process is successful. This ensures that only the authorized user can access their passwords.

   - **View All Password Labels (`v`):**  
     A list of all saved password labels is displayed, allowing users to quickly locate specific passwords. This method ensures that only the connected user's passwords are shown, as it decrypts the passwords using the user's master password.

   - **Delete a Password (`d`):**  
     Users can delete a stored password by providing its label. The password is then removed from the database, ensuring that sensitive information is securely erased. This operation is only successful if the decryption method works correctly, confirming that only the connected user can delete their own passwords.

   - **Quit the Program (`q`):**  
     Users can exit the program at any time when his connected.

3. **Password and database Encryption and Decryption:**
   - All passwords are encrypted using **AES-256-GCM** for secure storage. The encryption keys are derived from the user's master password using **PBKDF2**.
   - The master password itself is hashed using **SHA-256** with a unique salt to ensure that the password is never stored in plaintext.
   - The programm uses SQLCipher to encrypt your SQLite database, ensuring robust data protection.

4. **Educational Purpose:**
   - This password manager is primarily for educational purposes and is designed to demonstrate the steps involved in encryption and decryption.
   - All cryptographic operations, including key generation, hashing, and encryption, are displayed to help users understand the underlying processes.
   - The program should **not** be used for storing real passwords in a production environment.

5. **User Interface:**
   - The program runs in a terminal-based interface where users interact with simple text prompts.
   - After performing an action, the program will ask if the user wants to return to the options menu or exit the program.

6. **Error Handling:**
   - If any errors occur the program provides informative error messages and allows the user to retry the action.

This password manager is a hands-on tool designed to teach cryptographic principles for educational purpose.

## **Windows OS Setup Guide**
### **Prerequisites**

Before you begin, ensure the following tools are installed on your system:

1. **Microsoft C++ Build Tools**  
   - Required for compiling SQLite on Windows.  
   - Download from [Microsoft Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/).  
   - Install the "Desktop development with C++" workload.

2. **SQLite**  
   - SQLite may already be available on your system. Otherwise, download it from [SQLite Downloads](https://www.sqlite.org/download.html).
   - For Windows users, download the **Precompiled Binaries**:  
     - [Direct download link (64-bit DLL)](https://sqlite.org/2024/sqlite-dll-win-x64-3470000.zip).

3. **Rust** (Version 1.70 or higher)  
   - Install Rust from [Rust's official website](https://www.rust-lang.org).  

4. **Diesel CLI**  
   - For database migrations, Diesel CLI needs to be installed:  
     ```bash
     cargo install diesel_cli --no-default-features --features "sqlite-bundled"
     ```

5. **SQLCipher**  
   - SQLCipher adds encryption to SQLite.  
   - Instructions for compiling SQLCipher are included in the **Setup** section.

6. **ActiveState**  
   - Required for building SQLCipher.  
   - Create an account and download the platform from [ActiveState](https://www.activestate.com/platform/build-deploy/).

7. **OpenSSL**  
   - Required for SQLCipher compilation.  
   - Download **Win64OpenSSL-1_1_1w** from [Softpedia](https://www.softpedia.com/get/Programming/Components-Libraries/OpenSSL.shtml).

---

### **Setup Instructions for Windows**

1. **Clone the Repository**
   ```bash
   git clone [repository_link]
   cd password-manager
   ```

2. **Verify Rust Installation**  
   Run the following command to ensure Rust is installed:  
   ```bash
   rustc --version
   ```

3. **Install SQLCipher**
   - Clone the SQLCipher repository:
     ```bash
     git clone https://github.com/sqlcipher/sqlcipher.git
     cd sqlcipher
     ```
   - Edit the `Makefile.msc` file:
     - Open the file in a text editor (e.g., VS Code).
     - Update the following lines:
       ```makefile
       TCC = $(TCC) -DSQLITE_TEMP_STORE=1
       ```
       **Change to:**
       ```makefile
       TCC = $(TCC) -DSQLITE_TEMP_STORE=2 -DSQLITE_HAS_CODEC -I"C:\Program Files\OpenSSL-Win64\include"
       ```
       **AND add under**
       ```makefile
       !IF $(USE_ICU)!=0
        LTLIBPATHS = $(LTLIBPATHS) /LIBPATH:$(ICULIBDIR)
        LTLIBS = $(LTLIBS) $(LIBICU)
        !ENDIF
        # <</mark>>

        # You should not have to change anything below this line
        ###############################################################################
       ```
       **Change to:**
       ```makefile
       !IF $(USE_ICU)!=0
        LTLIBPATHS = $(LTLIBPATHS) /LIBPATH:$(ICULIBDIR)
        LTLIBS = $(LTLIBS) $(LIBICU)
        !ENDIF
        # <</mark>>
       LTLIBPATHS = $(LTLIBPATHS) /LIBPATH:"C:\Program Files\OpenSSL-Win64\lib\VC\static"
       LTLIBS = $(LTLIBS) libcrypto.lib libssl.lib ws2_32.lib shell32.lib advapi32.lib gdi32.lib user32.lib crypt32.lib
        # You should not have to change anything below this line
        ###############################################################################
       ```
   - Run the `Makefile.msc`:
     ```bash
     nmake /f Makefile.msc
     ```

4. **Add Environment Variables**  
   - Update your **User Path** environment variables:
     - Add `C:\Program Files\OpenSSL-Win64` to your `PATH`.
     - Add the SQLCipher directory to your `PATH`.
     - Set `OPENSSL_DIR` to `C:\Program Files\OpenSSL-Win64`.

5. **Configure Rust for Windows**
   - Set Rust to use the MSVC toolchain:
     ```bash
     rustup default stable-x86_64-pc-windows-msvc
     rustup target add x86_64-pc-windows-msvc
     ```

6. **Install Diesel CLI**
   ```bash
   cargo install diesel_cli --no-default-features --features "sqlite-bundled"
   ```

7. **Create a Folder for the Database**
   ```bash
   mkdir db
   ```

8. **Configure .env file**  
   - Create a `.env` file in the root directory:
     ```env
     DATABASE_URL=db/database.db
     DATABASE_PASSWORD=your_secure_password
     ```
   - Update the `diesel.toml` file to point to the migrations directory:
     ```toml
     [migrations_directory]
     dir = "C:/path/to/password-manager/migrations"
     ```
     You can refer to the `.env.example` file for guidance if needed.

9. **Build and Run the Project**
   ```bash
   cargo run
   ```

## **Linux OS Setup Guide**
### **Prerequisites**

1. **Rust** (Version 1.70 or higher)  
   - Install Rust from [Rust's official website](https://www.rust-lang.org).  
   - Run the following command to install Rust:  
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```

2. **Diesel CLI**  
   - For database migrations, Diesel CLI needs to be installed:  
     ```bash
     cargo install diesel_cli --no-default-features --features "sqlite-bundled"
     ```

3. **SQLCipher**  
   - SQLCipher adds encryption to SQLite.  
   - Run the following command to install SQLCipher:
   ```bash
   sudo apt install sqlcipher
   ```

### **Setup Instructions for Linux**

1. **Clone the Repository**
   ```bash
   git clone [repository_link]
   cd password-manager
   ```

2. **Verify Rust and SQLCipher Installation**  
   Run the following command to ensure Rust and SQLCipher are installed:  
   ```bash
   rustc --version
   sqlcipher --version
   ```

3. **Install Diesel CLI**
   ```bash
   cargo install diesel_cli --no-default-features --features "sqlite-bundled"
   ```

4. **Create a Folder for the Database**
   ```bash
   mkdir db
   ```

5. **Configure .env file**  
   - Create a `.env` file in the root directory:
     ```env
     DATABASE_URL=db/database.db
     DATABASE_PASSWORD=your_secure_password
     ```
   - Update the `diesel.toml` file to point to the migrations directory:
     ```toml
     [migrations_directory]
     dir = "/path/to/password-manager/migrations"
     ```
     You can refer to the `.env.example` file for guidance if needed.

6. **Build and Run the Project**
   ```bash
   cargo run
   ```
---

## **Notes**

- **Database Encryption:** SQLCipher ensures your SQLite database is encrypted using industry-standard algorithms.  
- **Cross-Platform Compatibility:** Although this project works on Windows, it is recommended and easier to run on Linux.
- **Troubleshooting:**  
  - Ensure the correct paths are set in your environment variables.  
  - Use the official [SQLCipher Documentation](https://www.zetetic.net/sqlcipher/) and [Diesel Documentation](https://diesel.rs/) for additional guidance.
