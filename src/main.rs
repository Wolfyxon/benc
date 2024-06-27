mod benc;

use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;

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

fn input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read stdin");

    buf = buf.replace("\n", "");

    return buf;
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
        },

        Option {
            alias: "encrypt_file",
            description: "Encrypts a file",
            minimum_args: 1,
            usage: "<file path> [password]",
            callback: encrypt_file_option
        }
    ];
}

fn help_option(args: Vec<String>) {
    print_help();
}

fn encrypt_text_option(args: Vec<String>) {
    let text = args.join(" ");
    
    println!("Enter password: ");

    let password = input();

    unsafe {
        println!("{}", benc::encrypt_string(text, password));
    }
}

fn decrypt_text_option(args: Vec<String>) {
    let text = args.join(" ");
    
    println!("Enter password: ");

    let password = input();

    unsafe {
        println!("{}", benc::decrypt_string(text, password));
    }
}

fn encrypt_file_option(args: Vec<String>) {
    let path = &args[0];
    let mut password = String::new();

    let mut input_file = std::fs::OpenOptions::new()
                        .read(true)
                        .open(path)
                        .expect(format!("Cannot open input {}", path).as_str());

    println!("Encrypting file: '{}'", path);

    if(args.len() > 1) {
        password = args[1].clone();
    } else {
        println!("Enter password: ");
        password = input();
    }

    let mut input_buff: Vec<u8> = Vec::new();
    input_file.read_to_end(&mut input_buff).expect("Failed to read file");
    assert!(input_buff.len() != 0, "Input buffer is 0 bytes");
    
    println!("Output buffer read, {} bytes", input_buff.len());

    let mut output_file = std::fs::OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open(path)
                        .expect(format!("Cannot open output {}", path).as_str());

    let output_buff = benc::encrypt(input_buff, password.as_bytes().to_vec());
    println!("Output buffer computed, {} bytes", output_buff.len());
    assert!(output_buff.len() != 0, "Output buffer is 0 bytes");

    println!("Writing to file");
    output_file.write_all(&output_buff).expect("Failed to write buffer");

    println!("File '{}' encrypted successfully", path);

}