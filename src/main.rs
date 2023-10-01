use std::io::{self, Write};
use std::fs::{self,File, *};
use std::io::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct BasicAccountMembers {
    account_holder: String,
    id: i16,
}

impl BasicAccountMembers {
    fn increment_id(&mut self) {
        self.id += 1;
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
// #[derive(Debug)]
struct Account {
    id: i16,
    account_holder: String,
    balance: f32,
}

impl Account {
    fn deposit(&mut self) {
        print!("Enter deposit amount: ");
        let _ = io::stdout().flush();
        let deposit_amount = get_user_float();
        self.balance += deposit_amount;
    }

    fn withdraw(&mut self) {
        print!("Enter withdraw amount: ");
        let _ = io::stdout().flush();
        let withdraw_amount = get_user_float();
        self.balance -= withdraw_amount;
    }
}
fn save_account_to_json(data: &Account) -> io::Result<()> {
    let json_string = serde_json::to_string(data)?;    
    let file_name = format!("AccountJsonfiles\\{}{}.json", data.account_holder, data.id.to_string());
    let mut file = File::create(file_name)?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}

fn save_id_struct_to_json(data: &BasicAccountMembers) -> io::Result<()> {
    let json_string = serde_json::to_string(data)?;
    let file_name = format!("account_id\\{}.json", data.account_holder);
    let mut file = File::create(file_name)?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}


fn get_user_integer() -> i16 {
    let mut _int: i16 = 0;
    let mut user_input_int = String::new();
    io::stdin()
        .read_line(&mut user_input_int)
        .expect("failed to read input");
    let trimmed_input = user_input_int.trim();  
    let parsed_user_input_age: Result<i16,std::num::ParseIntError> = trimmed_input.parse();
    match parsed_user_input_age {
        Ok(parsed_number) => {
            // println!("parsed_number: {}", parsed_number);
            _int = parsed_number;
        }
        Err(_i16) => {
            println!("failed to parse the string to integer.");
        }
    }    
    return _int;
}

fn get_user_float() -> f32 {
    let mut _flt: f32 = 0.0;
    let mut user_input_flt = String::new();
    io::stdin()
        .read_line(&mut user_input_flt)
        .expect("failed to read input");
    let trimmed_input = user_input_flt.trim();  
    let parsed_user_input_flt: Result<f32,std::num::ParseFloatError> = trimmed_input.parse();
    match parsed_user_input_flt {
        Ok(parsed_number) => {
            // println!("parsed_number: {}", parsed_number);
            _flt = parsed_number;
        }
        Err(_f32) => {
            println!("failed to parse the string to integer.");
        }
    }    
    return _flt;
}


fn get_user_string() -> String {
    let mut user_input_string = String::new();
     
    let _ = io::stdout().flush().unwrap();
    
    io::stdin()
        .read_line(&mut user_input_string)
        .expect("failed to read input");
    let trimmed_input = user_input_string.trim().to_string();
   
    trimmed_input
}
// creates account struct, taking signed 16 bit integer parameter
fn create_account(account_id: i16) -> Account {
    
    print!("Enter account holders name: ");
    let _ = io::stdout().flush(); // flush newline character
    let name:&str = &get_user_string(); // call for user defilned string variable
    print!("enter account balance: ");
    let _ = io::stdout().flush(); // flush newline character
    let _balance: f32 = get_user_float(); // call for user defined float variable
    // create struct from above variables and function parameter
    let account = Account {
        id: account_id,
        account_holder:String::from(name),
        balance: _balance,
    };
    // save the account struct as a json file in memory
    let _ = save_account_to_json(&account);
    return account; // return account struct
}
// creates Account struct in the same manner as the id tracker on line 144 below.
fn load_account_from_json(file_name: String) -> io::Result<Account> {
    println!("loading account from json.......");
    let full_path = format!("AccountJsonFiles/{}", file_name);
    let mut file = File::open(full_path)?;

    let mut json_string = String::new();
    file.read_to_string(&mut json_string)?;

    let account: Account = serde_json::from_str(&json_string)?;

    Ok(account)
}
// open the specified json file, creating a json string from the file contents, and create a struct
fn load_account_id_tracker_struct_from_json() -> io::Result<BasicAccountMembers> {
    
    let full_path = String::from("account_id/current_id.json"); // file path string
    let mut file = File::open(full_path)?; // create file from opening file at the given path
    let mut json_string = String::new(); // create empty string
    file.read_to_string(&mut json_string)?; // read the fill into the json string
    let current_id: BasicAccountMembers = serde_json::from_str(&json_string)?; // create a struct from the json string
    Ok(current_id) // return resulting struct to be checked and added to a vector
}
// call the above function and check for successfull creation of the struct
fn load_account_id_tracker_to_vector() -> io::Result<Vec<BasicAccountMembers>> {
    let mut current_account_id = Vec::new(); // empty vector to populate
    // check the struct returned from the called function.
    match load_account_id_tracker_struct_from_json() {
        Ok(current_id_struct) => {
            println!("loaded existing json");
            // if json exists, and creation of struct from json is successfull
            // push struct to vector
            current_account_id.push(current_id_struct);
        }
        Err(_err) => {
            // if no json exists, creation of the struct will fail, loading default struct into the vector 
            println!("No existing json, loaded default json");
            let basic_account_members: BasicAccountMembers = BasicAccountMembers { account_holder:String::from("current_id"),id: 0 };
            current_account_id.push(basic_account_members);
        }
    }
    Ok(current_account_id) // return resulting vector of account structs to be checked 
}

// generate a vector of strings, each being a json file found in the designated folderr
fn generate_list_of_account_jsons() ->io::Result<Vec<String>> {
    let mut json_files = Vec::new(); // vector to populate
    // reads the files at the specified directory
    for entry in fs::read_dir("AccountJsonFiles")? {
        let entry = entry?; // load the successful result of reading the read_dir entry into the entry variable.
        let path = entry.path(); // load the file path to the entry into the path variable
        // check for files of a specific extension: json
        if let Some(extension) = path.extension() { 
            if extension == "json" { 
                if let Some(file_name) = path.file_name(){
                    if let Some(file_str) = file_name.to_str() {
                        json_files.push(file_str.to_string());
                    }
                }
            }
        }
    }
    
    Ok(json_files)// return resulting vector of strings
}
// gets a generated vector of json files in a given folder
// and steps through the vector of files creating an account struct
// for each json file in the vector
fn load_all_json_files() -> io::Result<Vec<Account>> {
    let mut accounts = Vec::new(); // empty vector to populate
    // step through each json file in the vector
    for file in generate_list_of_account_jsons()? { //generate the vector of files
        println!("{}", file); // print name of loaded file
        // check for correct loading of json into account with call to the 
        // load_account_from_json function.
        match load_account_from_json(file) {
            Ok(loaded_account) => {
                let account1 = loaded_account.clone();
                accounts.push(account1); // populate accounts vector with generated structs
            }
            Err(err) => eprintln!("Error loading account from JSON: {}", err),
        }
    }

    Ok(accounts) // return resulting vector of Account structs
}

// take a reference to a mutable vector of accounts, and iterate through to find
// and return a specific account struct based on the account id stuct member
fn find_account(accounts: &mut Vec<Account>) -> Option<&mut Account> {
    print!("Enter account id to search for: ");
    let _ = io::stdout().flush(); // flushes newline character
    let account_id = get_user_integer(); // takes user input and returns 16 bit signed integer
    // steps through accounts in the vector until it finds the 
    // unique account id entered by the user
    for account in accounts.iter_mut() {
        if account.id == account_id {
            return Some(account);
        }
    }
    None // return None if no account found
}
// generate and return a 2 element tuple of a vector of "Account" structs, and a vector of "basis account members" structs
fn program_startup() ->(Vec<Account>,Vec<BasicAccountMembers>) {
    // check for json folders, create folders if missing. 
    check_for_json_folders();
    // create vector of account structs from existing json files, return type is a "Result".
    // which is a special return type for I/O operations, which this is as the vector of structs
    // is being created from json files in memory.
    let account_results = load_all_json_files();
    // check that the json files were correctly loaded into a vector of account structs
    let accounts = match account_results {
        // return vector of accounts, not of result type if the conversion was successfull
        Ok(accounts) => {
            accounts
        }
        Err(err) => {
            // return an empty vector if the conversion was unsuccessful
            eprintln!("Error loading accounts from JSON: {}", err);
            Vec::new()  // Return an empty vector or handle the error in another way
        }
    };  
    // perform the same operations on a json file in a different project directory
    let account_id_results = load_account_id_tracker_to_vector();

    let current_id = match account_id_results {
        Ok(current_id) => {
            current_id
        }
        Err(err) => {
            eprintln!("Error loading id struct from JSON: {}", err);
            Vec::new()
        }
    };

    let return_tuple = (accounts, current_id);
    return return_tuple;

}

fn delete_account(accounts: &mut Vec<Account> ) -> std::io::Result <()> {
    print!("Enter name of account holder to delete: ");
    let _ = io::stdout().flush();
    let account_holder_to_delete = get_user_string();
    print!("Enter id of account to delete: ");
    let _ = io::stdout().flush();
    let account_id_to_delete = get_user_integer();
    // remove account from Accounts Struct
    accounts.retain(|account| account.account_holder != account_holder_to_delete);
    // remove json file
    let file_name_to_delete = format!("AccountJsonfiles\\{}{}.json", account_holder_to_delete, account_id_to_delete.to_string());
    let _ = fs::remove_file(file_name_to_delete);

    Ok(())

}
fn menu(accounts: &mut Vec<Account>, current_id: &mut Vec<BasicAccountMembers>) {
    // let mut current_account_id = load_account_id_tracker_to_vector();
    
    let mut user_selection: i16;
    loop {
        println!("1. Create New Account.");
        println!("2. lookup Account.");
        println!("3. Delete Account.");
        println!("4. Make deposit.");
        println!("5. Make withdrawal.");
        println!("6. Display all accounts.");
        println!("0. Quit.");
    
        user_selection = get_user_integer();
        
        // quit
        if user_selection == 0 {
            break
        // create new account
        } else if user_selection == 1 {
            let account: Account = create_account(current_id[0].id);
            accounts.push(account);
            current_id[0].increment_id();
            let _ = save_id_struct_to_json(&current_id[0]);
        // display account details
        } else if user_selection == 2 {
            if let Some(account_to_list_details) = find_account(accounts) {
                println!("account holder: {}", account_to_list_details.account_holder);
                println!("account id: {}", account_to_list_details.id);
                println!("account balance: {}", account_to_list_details.balance);
            }
        // delete account
        } else if user_selection == 3 {
            let _ = delete_account(accounts);
        // deposit
        } else if user_selection == 4 { 
            if let Some(account_to_modify) = find_account(accounts) {
                account_to_modify.deposit();
                let _ = save_account_to_json(&account_to_modify);
            }
        // withdraw
        } else if user_selection == 5 {
            if let Some(account_to_modify) = find_account(accounts) {
                account_to_modify.withdraw();
                let _ = save_account_to_json(&account_to_modify);
            }
        // display all account id and holder
        } else if user_selection == 6 { 
            println!("Total number of accounts: {}", accounts.len());
            for account in &mut *accounts {
                println!("Account ID: {} Account holder: {} ", account.id, account.account_holder);
                // println!();
            }
        // error
        } else {
            println!("invalid input.");
        }
    }

}

fn create_folder(path: &str) {
    fs::create_dir(path).expect("Failed to create folder");
}
fn folder_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn check_for_json_folders() {
    let folder1 = String::from("account_id");
    let folder2 = String::from("AccountJsonFiles");
    if !folder_exists(&folder1) {
        create_folder(&folder1);
        println!("Folder created: {}", folder1);
    } else {
        println!("Folder already exists: {}", folder1);
    }

    if !folder_exists(&folder2) {
        create_folder(&folder2);
        println!("Folder created: {}", folder2);
    } else {
        println!("Folder already exists: {}", folder2);
    }

}
fn main() {
    // 
    
    let (mut accounts, mut current_id) = program_startup();
    let _ = menu(&mut accounts, &mut current_id);

  



    
    
    
}

