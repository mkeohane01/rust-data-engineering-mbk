/*

To run:

cargo run --  --message "Off to the bunker. Every person for themselves" --encrypt --shift 10

To decrypt:

cargo run --  --message "Ypp dy dro lexuob. Ofobi zobcyx pyb drowcovfoc" --decrypt --shift 10

To encrypt using piglatin:

cargo run --  --message "Off to the bunker. Every person for themselves" --piglatin-encrypt

*/


use caeser_cipher_cli::{decrypt, encrypt, piglatin};
use clap::Parser;

/// CLI tool to encrypt and decrypt messages using the caeser cipher
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Encrypt the message
    #[arg(short, long)]
    encrypt: bool,

    /// decrypt the message
    #[arg(short, long)]
    decrypt: bool,

    /// Encrypt the message using piglatin
    #[arg(short, long)]
    piglatin: bool,

    /// The message to encrypt or decrypt
    #[arg(long)]
    message: String,

    /// The shift to use for the cipher
    /// Must be between 1 and 25, the default is 3
    #[arg(short, long, default_value = "3")]
    shift: u8,
}

// run it
fn main() {
    let args = Args::parse();
    if args.encrypt {
        println!("{}", encrypt(&args.message, args.shift));
    } else if args.decrypt {
        println!("{}", decrypt(&args.message, args.shift));
    } else if args.piglatin {
        println!("{}", piglatin(&args.message));
    } else {
        println!("Please specify either --encrypt or --decrypt or --piglatin");
    }
}
