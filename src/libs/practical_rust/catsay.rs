use colored::Colorize;
use exitfailure::ExitFailure;

fn std_version()
{
    let message = std::env::args().nth(1)
        .expect("missing the message. Usage: catsay <message>")
        ;
    println!("{}", message);
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\");
    println!("    ( o o )");
    println!("    =( I )=");
}

fn structopt_version()
{
    extern crate structopt;
    extern crate colored;

    use structopt::StructOpt;
    use colored::*;

    #[derive(StructOpt)]
    struct Options {
        #[structopt(default_value="Meow!")]
        /// What does the cat say?
        message: String,

        #[structopt(short="d", long="dead")]
        /// Make the cat appear dead.
        dead: bool,
    }

    let options = Options::from_args();
    let message = options.message;
    let eye     = if options.dead {"x"} else {"o"};

    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.");
    }

    println!(r#"{}"#, message
        .bright_yellow().underline().on_purple()
    );
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\");
    println!("    ( {eye} {eye} )", eye=eye.red());
    println!("    =( I )=");
}

//fn structopt_version2() -> Result<(), Box<dyn std::error::Error>>
//fn structopt_version2() -> Result<(), failure::Error>
fn structopt_version2() -> Result<(), ExitFailure>
{
    extern crate structopt;
    extern crate colored;
    extern crate failure;

    use structopt::StructOpt;
    use colored::*;
    use failure::ResultExt;

    #[derive(StructOpt)]
    struct Options {
        #[structopt(default_value="Meow!")]
        /// What does the cat say?
        message: String,

        #[structopt(short="d", long="dead")]
        /// Make the cat appear dead.
        dead: bool,

        #[structopt(short="f", long="file", parse(from_os_str))]
        /// Load the cat picture from the specified file.
        catfile: Option<std::path::PathBuf>,
    }

    let options = Options::from_args();
    let message = options.message;
    let eye     = if options.dead {"x"} else {"o"};

    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.");
    }

    match &options.catfile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path)
                .with_context(|_|format!("could not read file: {:?}", path))?;
//            let cat_template = std::fs::read_to_string(path)?;
//            let cat_template = std::fs::read_to_string(path)
//                .expect(&format!("could not read file {:?}", path));
            let cat_picture = cat_template.replace("{eye}", eye);
            println!("{}", &cat_picture);
        },
        None => {
            println!(r#"{}"#, message
                .bright_yellow().underline().on_purple()
            );
            println!(" \\");
            println!("  \\");
            println!("     /\\_/\\");
            println!("    ( {eye} {eye} )", eye=eye.red());
            println!("    =( I )=");
        },
    }
    Ok(())
}

fn structopt_version3() -> Result<(), ExitFailure>
{
    extern crate structopt;
    extern crate colored;
    extern crate failure;

    use structopt::StructOpt;
    use colored::*;
    use failure::ResultExt;
    use std::io::{self, Read};

    #[derive(StructOpt)]
    struct Options {
        #[structopt(default_value="Meow!")]
        /// What does the cat say?
        message: String,

        #[structopt(short="d", long="dead")]
        /// Make the cat appear dead.
        dead: bool,

        #[structopt(short="f", long="file", parse(from_os_str))]
        /// Load the cat picture from the specified file.
        catfile: Option<std::path::PathBuf>,

        #[structopt(short="i", long="stdin")]
        /// Read message from STDIN instead of the argument
        stdin: bool,
    }

    let options = Options::from_args();
    let mut message = String::new();
    let eye     = if options.dead {"x"} else {"o"};

    if options.stdin {
        io::stdin().read_to_string(&mut message);
    } else {
        message = options.message;
    }

    if &message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.");
    }

    match &options.catfile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path)
                .with_context(|_|format!("could not read file: {:?}", path))?;
//            let cat_template = std::fs::read_to_string(path)?;
//            let cat_template = std::fs::read_to_string(path)
//                .expect(&format!("could not read file {:?}", path));
            let cat_picture = cat_template.replace("{eye}", eye);
            println!("{}", &cat_picture);
        },
        None => {
            println!(r#"{}"#, message
                .bright_yellow().underline().on_purple()
            );
            println!(" \\");
            println!("  \\");
            println!("     /\\_/\\");
            println!("    ( {eye} {eye} )", eye=eye.red());
            println!("    =( I )=");
        },
    }
    Ok(())
}

pub fn test()
{
//    std_version();
//    structopt_version();
//    structopt_version2().expect("err occur");
    structopt_version3().expect("err occur");
}