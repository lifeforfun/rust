mod rlibs;

use std::io;
use std::process;

fn read_input() {
    println!("Input command：");

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            if input.len()<1 {
                println!("Please input command");
                process::exit(rlibs::defines::CODE_FAILED);
            }
            let cmd = &input[..input.len()-1];
            match  cmd{
                ".exit" => {
                    process::exit(rlibs::defines::CODE_SUCCESS);
                }
                _ => {
                    println!("Unrecognized command '{}'", cmd);
                    process::exit(rlibs::defines::CODE_FAILED);
                }
            }
        }
        Err(err) => {
            println!("Failed to read command：{}", err);
        }
    }
}

fn main() {
    read_input();
}