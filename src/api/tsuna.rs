use clap::{arg, Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version)]
#[clap(
    about = "Tsunagu Key-pair generation utility...",
    long_about = "Tsungau is blockchain which gives 100000 TPS"
)]

pub struct Tsunagu {
    #[clap(subcommand)]
    pub command: Tsuna,
}

#[derive(Debug, Subcommand)]
pub enum Tsuna {
    /// Generate new keypair file from a random seed phrase and optional BIP39 passphrase
    New(New),
    /// Display the pubkey from a keypair file
    Pubkey(PubKey),
    /// Recover keypair from seed phrase and optional BIP39 passphrase
    Recover(Mnemonics),
   /// get more keypair from your mnemonic
    Temp(Data),
}
//generate_keypair_forspecificindex
// mnemonic either user can enter or get from id.json
// path //Enter how many keys you want to derive
// _passprase enter password if you used
#[derive(Debug, Args)]
pub struct Data {
    ///Enter Mnemonic if you want to generate keypair from that
    #[arg(short = 'm', long = "mnemonic")]
    pub mnemonic: Option<String>,
    ///Number of keys you want to derive from mnemonic
    pub no_keys: Option<String>,
}
#[derive(Debug, Args)]
pub struct New {
    // #[clap(subcommand)]
    ///Overwrtie the output file if it exits
    #[arg(short = 'f', long = "force")]
    pub force: bool,

    ///Path to generated file
    #[arg(short = 'o', long = "outfile")]
    pub outfile: Option<String>,
}

#[derive(Debug, Args)]
pub struct PubKey {
    ///Filepath or URL to a keypair
    pub keypair: Option<String>,
}

// #[derive(Debug, Args)]
// pub struct Recover{
//     #[clap(subcommand)]
//     /// `prompt:` URI scheme or `ASK` keyword
//     pub command: Option<String>,
// }

#[derive(Debug, Subcommand)]
pub enum FromData {
    /// Provide private key
    PrivateKey(Privatekey),
    /// Provide Mnemonics
    Mnemonics(Mnemonics),
}

#[derive(Debug, Args)]
pub struct Privatekey {
    /// Provide private key for recovery
    pub from: String,
}

#[derive(Debug, Args)]
// #[arg(num_args(0..))]
pub struct Mnemonics {
    /// Provide Mnemonics key for recovery
    #[arg(short = 'm', long = "mnemo")]
    pub mnemo_string: String,
    #[arg(short = 'p', long = "path")]
    pub path: String,
    #[arg(short = 'f', long = "flag")]
    pub string: Option<String>,
}
