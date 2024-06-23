mod benc;

use std::env;
use std::io;

struct Option<'a> {
    alias: &'a str,
    usage: &'a str,
    description: &'a str,
    minimum_args: usize,
    callback: fn(args: Vec<String>)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if &args.len() <= &1 {
        print_help();
        return;
    }

    let alias = &args[1];

    let mut opt_args = args.clone();
    opt_args.remove(0);
    opt_args.remove(0);
    
    let opt_args_len = opt_args.len();

    for option in get_options() {
        if(option.alias == alias) {
            if opt_args_len < option.minimum_args {
                println!("This option requires at least {} arguments.", option.minimum_args);
                println!("{}", option.usage);
                return;
            }

            (option.callback)(opt_args.clone());
            return;
        }
    }

    println!("Unknown option '{}'", alias);
    print_help();
}

fn print_help() {
    println!("BENC - Basic Encryption");
    println!("https://github.com/Wolfyxon/benc");
    println!("NOTE: This is just a toy project, don't use it to encrypt important files as the algorithm is easy to break with brute force. \n");

    println!("Usage:");

    for option in get_options() {
        println!("  {}: {}\n    {}\n", option.alias, option.description, option.usage);
    }
}


fn get_options() -> Vec<Option<'static>> {
    return vec![
        Option {
            alias: "help",
            description: "Prints help",
            minimum_args: 0,
            usage: "",
            callback: help_option
        },

        Option {
            alias: "encrypt_text",
            description: "Encrypts plain text. You'll be prompted for the password",
            minimum_args: 1,
            usage: "<text...>",
            callback: encrypt_text_option
        },

        Option {
            alias: "decrypt_text",
            description: "Decrypts encrypted text. You'll be prompted for the password",
            minimum_args: 1,
            usage: "<encrypted text...>",
            callback: decrypt_text_option
        }
    ];
}

fn help_option(args: Vec<String>) {
    print_help();
}

fn encrypt_text_option(args: Vec<String>) {
    let text = args.join(" ");
    
    println!("Enter password: ");

    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read stdin");
    password = password.remove(password.len() - 1).to_string();

    unsafe {
        println!("{}", benc::encrypt_string(text, password));
    }
}

fn decrypt_text_option(args: Vec<String>) {
    let text = args.join(" ");
    
    println!("Enter password: ");

    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read stdin");
    password = password.remove(password.len() - 1).to_string();

    unsafe {
        println!("{}", benc::decrypt_string(text, password));
    }
}