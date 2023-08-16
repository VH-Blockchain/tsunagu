mod api;

use api::tsuna::{Tsuna, Tsunagu};
use clap::Parser;


fn main() {
    let tsunagu = Tsunagu::parse();
    // println!("Hello, world!");

    match &tsunagu.command   {
        Tsuna::New(_new) => {
            let force = _new.force.clone();
            let outfile = _new.outfile.clone();
            // println!("{},{}",force,outfile.unwrap());
            api::funktion::new_keypair(force,outfile.unwrap_or("".to_string()));
            // api::gennew::newerr();
        }
        Tsuna::Pubkey(_pubkey) => {
            api::funktion::pubkeyy();
        }
        Tsuna::Recover(_recover) => {
            let input_mnemo = _recover.mnemo_string.clone();
            let path = _recover.path.clone();
            // println!("Your Entered Mnemonics: {:?}",input_mnemo);
            api::funktion::recoverr(&input_mnemo,path);
        }
        Tsuna::Temp(_data) =>{
            api::funktion::getmultiplekeypair(_data.mnemonic.clone(),_data.no_keys.clone());
            println!("Hello World");
        }
        // Tsuna::Verify(verify) => println!("Verifying keypair file: {}", verify.keypair_file),

    }
        // api::funktion::new_keypair();
}
