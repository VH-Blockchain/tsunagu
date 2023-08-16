use rpassword::read_password;
use std::{env, fs};
use std::io::Read;
use std::{fs::File, io::Write, path::Path};

use crate::api::allfunk::*;
pub fn folder_creation()->String{
    let home_dir = env::var("HOME").unwrap();
    let p = String::from("Tsunagu");
    let data = format!("{home_dir}/.local/share/{p}");
    // println!("data{}",data)
    let ubanatu_share_dir = Path::new(&home_dir).join(&data);
    if !ubanatu_share_dir.exists() {
        let _t = fs::create_dir(ubanatu_share_dir.clone());        
     }
 
    return data
}
pub fn new_keypair(force: bool,outfile:String) {
    let filepath = format!("{}/id.json",folder_creation()); 
    let mut tsunid = Path::new(&outfile);
    if outfile.len()==0{
        tsunid = Path::new(filepath.as_str());
    }
    println!("Generating a new keypair \n");
    println!("For added security, enter a BIP39 passphrase \n");
    println!("NOTE! This passphrase improves security of the recovery seed phrase NOT the \nkeypair file itself, which is stored as insecure plain text \n");
    print!("BIP39 Passphrase (empty for none):");
    std::io::stdout().flush().unwrap();
    let password = read_password().unwrap();
    print!("Enter same passphrase again:");
    std::io::stdout().flush().unwrap();
    let pass = read_password().unwrap();
    println!();
    if password.eq(&pass) {
        let keypair = generate();
        let f = force || !tsunid.exists();
        if store_file(&keypair, f, tsunid) {
            println!(
                "=============================================================================="
            );
            println!("Public address: {:?}", keypair.public_address);
            println!(
                "=============================================================================="
            );
            println!("Save this seed phrase & your BIP39 passphrase to recover your account:");
            println!("Mnemonics: {}", keypair.mnemonics);
            println!(
                "=============================================================================="
            );
            println!("Secret key: {}", keypair.private_key);
            println!("Public Key: {}", keypair.public_key);
            println!("Path: {:?}", keypair.path);
            println!(
                "=============================================================================="
            );
        } else {
            print!("Already exists file here....")
        }
    } else {
        print!("Passphrases did not match");
    }
}

pub fn pubkeyy() {
    let mut file = File::open( format!("{}/id.json",folder_creation()).as_str())
        .expect("Failed to open file");
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)
        .expect("Failed to read file");

    let parsed_value: serde_json::Value =
        serde_json::from_str(&json_data).expect("Failed to parse JSON");
    if let Some(public_address) = parsed_value.get("public_address") {
        println!("Public Address is:{}", public_address);
    } else {
        println!("No Public Address found. Run \"tsu-key new\" command ");
    }
}

// pub fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }
pub fn getmultiplekeypair(mnemonic:Option<String>,no_of_pair:Option<String>){
//   let d = no_of_pair.unwrap_or_default().parse::<i32>().unwrap();
  println!("11{}",mnemonic.unwrap_or("0".to_string()).parse::<i32>().unwrap());
  for i in 0..3 {
    let path = format!("m/44'/60'/0'/0/{i}");
    println!("1");
    generate_keypair_forspecificindex("".to_string(),path,"".to_string());
  }
}
pub fn recoverr(input: &String,mut path: String) {
    path = format!("{}", path + "id.json");
    let file_path = Path::new(path.as_str());
    let recover_mnemo = input.replace(",", " ");
    println!("You Entered {}", recover_mnemo);
    let fetched_account = generate_from_mnemonic(recover_mnemo, "".to_string());
    println!("Your Tsunagu Account is\n{:#?}", fetched_account);
    store_file(&fetched_account, file_path.exists(), file_path);
    // let mut file = File::open("/home/vhits/Desktop/tsunagu-keygen/tsu-key/tsunid/tsun_id.json").expect("Failed to open file");
    // let mut json_data = String::new();
    // file.read_to_string(&mut json_data).expect("Failed to read file");

    // let parsed_value: serde_json::Value = serde_json::from_str(&json_data).expect("Failed to parse JSON");
    // if parsed_value["mnemonics"].as_array().unwrap() == &parsed_value {
    //     println!("{}", parsed_value);
    // } else {
    //     println!("Wrong input");
    // }
}

pub fn store_file(keypair: &Keypair, bpath: bool, path: &Path) -> bool {
    match bpath {
        true => {
            // println!("{:?}",path);
            let mut file = std::fs::File::create(path).unwrap();
            serde_json::to_writer_pretty(&mut file, &keypair).unwrap();
            return true;
        }
        false => return false,
    }
}
