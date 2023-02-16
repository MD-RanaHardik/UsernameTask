extern crate regex;
use regex::Regex;
mod utility;
use std::collections::HashMap;
mod username_validation;
mod password_validation;
use chrono;



pub struct UserDetails {
    Username: String,
    Password:String,
    Name: String,
    Address: String,
    Number: u128,
}
static  mut is_anyone_login:bool =false;



fn main() {
    

    let mut users: HashMap<String, UserDetails> = HashMap::new();
    
    let mut is_run_program = true;


    while is_run_program {
        println!(" \n\n 1. For ADD new account : ANA \n 2. For check username availability : CUA \n 3. For login : Login \n 4. For logout : Logout \n 5. For exit : Exit" );
        println!("Enter your operation name: ");
        let choice = utility::read_user_chois();

        match choice.as_str() {
            "ANA" => {
                utility::signup(&mut users);
            }
            "CUA" => {
                username_validation::check_username_availibility(&mut users);
            }
            "Login" => {
                utility::login(&mut users);
            }
            "Logout" => {
                utility::logout();
            }
            "UD"=>{}
            "DA"=>{}
            "Exit" => {
                is_run_program = false;
                println!("Thankyou for visiting our site.");
            }
            _ => println!("Please choose from above option"),
        }
    }

    
}
