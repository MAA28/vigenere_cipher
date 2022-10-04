use clap::{Args, Parser, Subcommand};

#[derive(Parser)] // requires `derive` feature
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Output more information
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Encode a text with the vigenere cipher.
    Encode(EncodeOptions),

    /// Decode a text, encoded with the vigenere cipher.
    Decode(DecodeOptions),

    /// Crack a text, encoded the vigenere cipher. (If you have the key, use the decode option!)
    Crack(CrackOptions),
}

#[derive(Args, Debug)]
pub struct EncodeOptions {
    /// The text that is to be encoded. **Note: Only lowercase letters are excepted!**
    #[arg(short = 't', long)]
    pub text: String,

    /// The key with which the text is to be encoded. **Note: Only lowercase letters are excepted!**
    /// lowercase letter!**
    #[arg(short = 'k', long)]
    pub key: String,
}

#[derive(Args, Debug)]
pub struct DecodeOptions {
    /// The text that is to be decoded. **Note: Only lowercase letters are excepted!**
    #[arg(short = 't', long)]
    pub text: String,

    /// The key with which the text is to be decrypted. **Note: Only lowercase letters are excepted!**

    #[arg(short = 'k', long)]
    pub key: String,
}

#[derive(Args, Debug)]
pub struct CrackOptions {
    /// The text that is to be cracked. **Note: Only lowercase letters are excepted!**
    #[arg(short = 't', long)]
    pub text: String,
    
    /// The length of the key with which the text was encoded. 
    #[arg(short, long)]
    pub length: Option<usize>
}
