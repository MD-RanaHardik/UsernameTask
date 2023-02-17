extern crate regex;
use rand::distributions::{Alphanumeric, DistString};

use crate::{is_anyone_login, password_validation, username_validation, UserDetails};

use std::{collections::HashMap, io};

use local_ip_address::local_ip;

pub fn read_input(type_of_input: String) -> String {
    println!("Enter {} :", type_of_input);
    let mut data = String::new();
    let _input = io::stdin().read_line(&mut data);
    return data.trim().to_string();
}

pub fn read_user_chois() -> String {
    let mut choice = String::new();
    let _input = io::stdin().read_line(&mut choice);
    return choice.trim().to_string();
}

pub fn check_and_add_user(
    users: &mut HashMap<String, UserDetails>,
    username: &String,
    login_logout_log: &mut HashMap<String, Vec<String>>,
    unapproved_accounts: &mut Vec<String>,
) {
    if users.contains_key(username.as_str()) {
        propper_output("Username is already exist");
    } else {
        let mut is_read_password = true;

        while is_read_password {
            let password = &read_input("password".to_string());
            let password_msg = password_validation::validate_password(password);

            match password_msg.as_str() {
                "Valid" => {
                    let userdetails = UserDetails {
                        Username: username.to_string(),
                        Password: password.to_string(),
                        Name: String::from("Default"),
                        Address: String::from("Unknown"),
                        Number: 1234567890,
                        is_admin: false,
                    };

                    users.insert(username.to_string(), userdetails);
                    generate_log(&username, "Login".to_string(), login_logout_log);
                    unapproved_accounts.push(username.to_string());
                    propper_output(
                        format!("Account is successfully generated using {}.", username).as_str(),
                    );

                    is_read_password = false;
                }
                _ => propper_output(format!("Error: {}", password_msg).as_str()),
            }
        }
    }
}

pub fn signup(
    users: &mut HashMap<String, UserDetails>,
    login_logout_log: &mut HashMap<String, Vec<String>>,
    unapproved_accounts: &mut Vec<String>,
) {
    let username = read_input("username".to_string());
    let result = username_validation::validate_username(&username);

    match result.as_str() {
        "Valid" => check_and_add_user(users, &username, login_logout_log, unapproved_accounts),
        _ => {
            println!("Error: {}", &result)
        }
    }
}

pub fn login(
    users: &mut HashMap<String, UserDetails>,
    login_logout_log: &mut HashMap<String, Vec<String>>,
    is_admin: bool,
    unapproved_accounts: &mut Vec<String>,
) {
    unsafe {
        if is_anyone_login && !is_admin {
            propper_output("User already logged in")
        } else {
            let mut password_attempt = 0;
            while password_attempt < 3 {
                let username = read_input("username".to_string());
                let password = read_input("password".to_string());

                match users.get(&username) {
                    Some(value) => {
                        if password == value.Password {
                            is_anyone_login = true;
                            generate_log(
                                &username,
                                "Created new account".to_string(),
                                login_logout_log,
                            );
                            propper_output("Login successfully");
                            if is_admin {
                                approve_accounts(users, unapproved_accounts);
                            }

                            break;
                        } else {
                            propper_output("Please check username and password");
                            password_attempt += 1;
                            if password_attempt >= 3 {
                                propper_output("Login attempt reached");
                                break;
                            }
                        }
                    }
                    None => {
                        propper_output("Please check username and password");

                        if password_attempt >= 3 {
                            propper_output("Login attempt reached");
                            break;
                        }
                    }
                }
            }
        }
    }
}

pub fn logout() {
    unsafe {
        if is_anyone_login {
            is_anyone_login = false;
            propper_output("You're successfully logged out");
        } else {
            propper_output("Please login before logout.")
        }
    }
}

pub fn validate_and_change_details(
    username: &String,
    users: &mut HashMap<String, UserDetails>,
    login_logout_log: &mut HashMap<String, Vec<String>>,
) {
    match users.get(username) {
        Some(_value) => {
            let name = read_input("name".to_string());
            let address = read_input("address".to_string());
            let number = read_input("number".to_string());

            if !name.is_empty() {
                (users.get_mut(username).unwrap()).Name = name;
            }
            if !address.is_empty() {
                (users.get_mut(username).unwrap()).Address = address;
            }
            if !number.to_string().is_empty() && number.to_string().len() == 10 {
                (users.get_mut(username).unwrap()).Number = number.parse::<u128>().unwrap();
            } else {
                propper_output("Please number must be equals to 10");
            }

            println!("{:?}", users.get(username));
            generate_log(username, "Details updated".to_string(), login_logout_log);
            propper_output("Details updated");
        }
        None => propper_output(format!("Account not exist for {} username ", username).as_str()),
    }
}

pub fn update_details(
    users: &mut HashMap<String, UserDetails>,
    login_logout_log: &mut HashMap<String, Vec<String>>,
) {
    unsafe {
        if is_anyone_login {
            let username = read_input("username".to_string());
            validate_and_change_details(&username, users, login_logout_log);
        } else {
            propper_output("Please login before going to change anything.")
        }
    }
}

pub fn delete_user(users: &mut HashMap<String, UserDetails>) {
    let username = read_input("username".to_string());
    let capcha = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);

    match users.get(&username) {
        Some(_value) => {
            if _value.is_admin {
                println!("Capcha : {}", capcha);
                let user_writen_capcha = read_input("capcha".to_string());

                if user_writen_capcha == capcha {
                    users.remove(&username);
                    propper_output("Account successsfully deleted");
                    logout();
                } else {
                    propper_output("Please enter correct capcha code");
                }
            } else {
                propper_output("You can't delete account without a permission")
            }
        }
        None => propper_output("Please check username and password"),
    }
}

fn generate_log(
    username: &String,
    log_for: String,
    login_logout_log: &mut HashMap<String, Vec<String>>,
) {
    match login_logout_log.get(username) {
        Some(logs) => {
            let log = format!(
                "{} at : {} using {} this IP Address",
                log_for,
                chrono::Local::now(),
                local_ip().unwrap()
            );
            (login_logout_log.get_mut(username).unwrap()).push(log.to_string());
        }
        None => {
            let log = format!(
                "{} at : {} using {} this IP Address",
                log_for,
                chrono::Local::now(),
                local_ip().unwrap()
            );
            login_logout_log.insert(username.to_string(), vec![log]);
        }
    }
}

pub fn check_log(login_logout_log: &mut HashMap<String, Vec<String>>) {
    let username = read_input("username".to_string());

    match login_logout_log.get(&username) {
        Some(logs) => {
            println!("\nLogs of {}", username);
            let mut templog = logs.clone();
            templog.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());
            println!(
                "+{}+",
                String::from("-").repeat(templog.last().unwrap().len() + 10)
            );

            for log in logs {
                println!(
                    "|     {}{}|",
                    log,
                    String::from(" ").repeat(templog.last().unwrap().len() - log.len() + 5)
                );
            }

            println!(
                "+{}+",
                String::from("-").repeat(templog.last().unwrap().len() + 10)
            );
            drop(templog);
        }
        None => {
            propper_output("Please check username");
        }
    }
}

pub fn create_master_admin(users: &mut HashMap<String, UserDetails>) {
    let admin_details = UserDetails {
        Username: String::from("Admin@@123"),
        Password: String::from("Admin@@123"),
        Name: String::from("Admin"),
        Address: String::from("Ahm"),
        Number: 1111111111,
        is_admin: true,
    };
    users.insert("Admin@@123".to_string(), admin_details);
}

pub fn approve_accounts(
    users: &mut HashMap<String, UserDetails>,
    unapproved_accounts: &mut Vec<String>,
) {
    if unapproved_accounts.clone().is_empty() {
        propper_output("We don't have a any request");
    } else {
        propper_output(
            format!(
                "Total accounts to be approv : {}",
                unapproved_accounts.len()
            )
            .as_str(),
        );

        for (index, account) in &mut unapproved_accounts.clone().iter().enumerate() {
            match users.get(account) {
                Some(details) => {
                    println!("\n\nAccount : {}", index + 1);

                    propper_output(
                        format!(
                            "Username: {}\nName : {}\nAddress :{}\nNumber :{} ",
                            details.Username, details.Name, details.Address, details.Number
                        )
                        .as_str(),
                    );

                    println!("To approve this account press : A");
                    let mut choice = String::new();
                    io::stdin().read_line(&mut choice);

                    match choice.trim().to_string().as_str() {
                        "A" => {
                            (users.get_mut(&details.Username.to_string()).unwrap()).is_admin = true;
                            unapproved_accounts.remove(index);
                        }
                        _ => propper_output("Not selected anything"),
                    }
                }
                None => {}
            }
        }
    }
}

pub fn propper_output(input: &str) {
    if input.contains("\n") {
        let mut lines: Vec<&str> = input
            .split("\n")
            .filter(|a| !a.is_empty())
            .map(|a| a.trim())
            .collect();

        lines.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());

        println!(
            "+{}+",
            String::from("-").repeat(lines.last().unwrap().len() + 10)
        );

        for line in input
            .split("\n")
            .filter(|a| !a.is_empty())
            .map(|a| a.trim())
        {
            println!(
                "|{}{}{}|",
                String::from(" ").repeat(5),
                line,
                String::from(" ").repeat(lines.last().unwrap().len() - line.len() + 5)
            );
        }
        println!(
            "+{}+",
            String::from("-").repeat(lines.last().unwrap().len() + 10)
        );
    } else {
        println!("+{}+", String::from("-").repeat(input.len() + 10));
        println!(
            "|{}{}{}|",
            String::from(" ").repeat(5),
            input,
            String::from(" ").repeat(5)
        );
        println!("+{}+", String::from("-").repeat(input.len() + 10));
    }
}
