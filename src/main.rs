extern crate regex;
mod utility;
use std::{collections::HashMap, io};
mod password_validation;
mod username_validation;


// only admin approve for make another admin
// login attempts

#[derive(Debug)]
pub struct UserDetails {
    Username: String,
    Password: String,
    Name: String,
    Address: String,
    Number: u128,
    is_admin:bool,
}
static mut is_anyone_login: bool = false;


fn main() {
    let mut users: HashMap<String, UserDetails> = HashMap::new();
    let  mut login_logout_log:HashMap<String,Vec<String>> = HashMap::new();
    let mut unapproved_accounts:Vec<String> = vec![];

    utility::create_master_admin(&mut users);

    let mut is_run_program = true;
    while is_run_program {
        println!(" \n\n 1. For ADD new account \n 2. For check username availability \n 3. For login \n 4. For logout \n 5. For update details \n 6. For delete account \n 7. For check logs \n 8. For exit" );
        println!("Enter your option: ");
        let choice = utility::read_user_chois();

        match choice.as_str() {
            "1" => {
                utility::signup(&mut users,&mut login_logout_log,&mut unapproved_accounts);
            }
            "2" => {
                username_validation::check_username_availibility(&mut users);
            }
            "3" => {
                

                println!("1. For Normal User \n2. For Admin User");
                let mut user_type = utility::read_input("you choice".to_string());

                match user_type.as_str() {
                    "1"=>{utility::login(&mut users,&mut login_logout_log,false,&mut unapproved_accounts);},
                    "2"=>{utility::login(&mut users,&mut login_logout_log,true,&mut unapproved_accounts);}
                    _=>{println!("must be select from above option")}
                }
                
            }
            "4" => {
                utility::logout();
            }
            "5" => {
                utility::update_details(&mut users,&mut login_logout_log);
            }
            "6" => {
                utility::delete_user(&mut users);
            }
            "7" => {
                utility::check_log(&mut login_logout_log);
            }
            "8" => {
                is_run_program = false;
                println!("Thankyou for visiting our site.");
            }
            _ => println!("Please choose from above option"),
        }
    }
}
