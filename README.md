# Overview

The primary purpose of this project was to learn Rust and explore the benefits that it provides. Rust introduces the concept of ownership and borrowing, which is enforced by the compiler and prevents race conditions. In addition to learning Rust, this project dives into the concepts of saving program data to memory.

This software is an account management system, that allows the user to create accounts, holding a name of the account holder, an automatically generated account id, and an account balance. Accounts made during each session are saved to file as JSON files, and are loaded at each use. The program keeps track of what the next account id should be between sessions also, by saving the next account id in a JSON file and loading each session. The user can make deposits and withdrawals from accounts, look up an individual account or list off the account holder and id of all accounts. The user also has the ability to delete individual accounts by entering the name of the account holder and the account id when prompted.

''
[Software Demo Video](https://youtu.be/pFtEOJCNjIo)

# Development Environment

The code for this project was written in VScode. The software was written in the Rust programming language. Building, compiling and running was done with cargo which is part of [rustup](https://www.rust-lang.org/tools/install). The command "cargo build" builds the project file and "cargo run" runs the program. The rustup link above will take you to the rust-lang.org website to down RUSTUP-INIT.EXE which makes cargo available to run this software.

Rust is a language that emphasizes performance while enforcing memory safety. Due to the enforced memory safety, Rust is an excellent language for concurrent programming.

# Useful Websites
The following websites were extremely useful in building this project.

- [rust-lang.org](https://www.rust-lang.org/)
- [stackoverflow.com](https://stackoverflow.com/)
- [Rust tutorialspoint.com](https://www.tutorialspoint.com/rust/index.htm)

# Future Work


- Additional account attributes such as transaction History
- Additional methds to edit the account attributes    
- Option to delete all Accounts
- graphical elements
- Additional Error checking
