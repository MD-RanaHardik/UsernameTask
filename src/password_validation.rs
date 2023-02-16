use regex::Regex;
use std::{io};

pub fn validate_password(password: &String) -> String {
    
    if (password.is_empty()) {
        return String::from("Password can not be empty");
    } else if (password.len() < 8) {
        return String::from("Password length must be grater then or equal to 8");
    } else if (password.len() > 16) {
        return String::from("Password length must be less then or equal to 16");
    } else if (Regex::new("^_|^[0-9]").unwrap().is_match(password.as_str())) {
        return String::from("Password can not start with _ or number");
    } else if (!Regex::new("[0-9]{1,}").unwrap().is_match(password.as_str())) {
        return String::from("Password have at least one number");
    } else if (!Regex::new("[A-Z]{1,}").unwrap().is_match(password.as_str())) {
        return String::from("Password have at least one capital letter");
    } else if (!Regex::new("[a-z]{1,}").unwrap().is_match(password.as_str())) {
        return String::from("Password have at least one small letter");
    } else if (!Regex::new("[~@$#!%^&*()]{1,}")
        .unwrap()
        .is_match(password.as_str()))
    {
        return String::from("Username use have at least one spacial character like:: ~@$#!%^&*()");
    }

    return "Valid".to_string();
}


