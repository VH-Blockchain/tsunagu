// use bip39::{Language,Mnemonic, MnemonicType, Seed};
// use rand_core::OsRng;
// use std::{any::type_name, path::StripPrefixError, str::FromStr};
// use tiny_hderive::bip32::ExtendedPrivKey;
// use secp256k1::{SecretKey,PublicKey};
// use hex::ToHex;
// use k256::elliptic_curve::sec1::ToEncodedPoint;
// use sha3::{Digest,Keccak256};

// use std::fs::File;
// use std::io::{self, Read, Write};
// use std::path::Path;
// use std::result::Result;
// use std::fmt::Display;

// pub struct Xyz {
//     mnemonics: Mnemonic,
//     pubk: String,
//     seck: String,
//     pubaddr: String
// }

// pub fn newerr() {
//     let tsunid = Path::new("/home/vhits/Desktop/tsunagu-keygen/tsu-key/tsunid/tsun_id.json");
//     // if !std::path::Path::new(&tsunid).exists() {
//     //     std::fs::create_dir(tsunid).unwrap();
//     //   }
//     // let tsun_id = format!("{}/people.json", tsunid);
//     let tsun_path = tsunid.exists();
//     let mut vec1 = Vec::new();
//     // println!("{:?}", tsun_path);
//     match tsun_path {
//         false => {
//             let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
//             // println!("{}", mnemonic);
//             let seed = Seed::new(&mnemonic, "");
//             let a_str=vec1.push(&seed);
//             println!("{:?}", a_str);
//             let account = ExtendedPrivKey::derive(seed.as_bytes(), "m/44'/60'/0'/0/0").unwrap();
//             let secret =SecretKey::parse(&account.secret()).unwrap();
//             let public_key =PublicKey::from_secret_key(&secret);
//             let secret_key = secret.serialize();
//             let secret_key_hex= hex::encode(secret_key);
//             let pkpk = public_key.serialize_compressed();
//             let pub_key = k256::PublicKey::from_sec1_bytes(&pkpk).expect("failed parsing eth bytes");
//             let addr = Keccak256::digest(&pub_key.to_encoded_point(false).as_bytes()[1..]);
//             let public_address = eip55::checksum(&hex::encode(&addr[12..]));
//             let public_key_hex= hex::encode(pkpk);
//             let ff = hex::encode(&addr[12..]);
//             let p = Xyz{
//                 mnemonics: mnemonic,
//                 pubk: public_key_hex,
//                 seck: secret_key_hex,
//                 pubaddr: public_address
//             };
//             // write_file(tsunid, Xyz { mnemonics: mnemonic, pubk: public_key_hex, seck: secret_key_hex, pubaddr: public_address });
            

//             println!("==============================================================================");
//             println!("Save this seed phrase & your BIP39 passphrase to recover your account:");
//             println!("Mnemonic: {}", mnemonic.phrase());
//             println!("==============================================================================");
//             println!("Secret key: {:?}", secret_key_hex);
//             println!("Public Key: {:?}", public_key_hex);
//             println!("Public address: {:?}", public_address);
//             println!("==============================================================================");
//             println!("secret key: {:?}", secret_key);


//             // println!("Private Key: {:?}-{:?}%%", secret_key, print_type_of(&secret_key));
//             // println!("Public Key: {}-{:?}$$", public_key, print_type_of(&public_key));
// }
//         true => println!("Already exists file hereeeeee"),
// }
// }

// // pub fn write_file(tsunid: &Path, Xyz { mnemonics, pubk, seck, pubaddr }: Xyz) {
// //     let mut file = File::create(tsunid).unwrap();
// //     serde_json::to_writer_pretty(&mut file, &p);
// //     // file.write_all(Xyz).unwrap();
// //     println!("File created successfully")
// //     // let mut tsunid = File::create("/home/vhits/Desktop/tsunagu-keygen/tsu-key/tsunid/id.json").expect("Failed to create tsunid");
// //     // let abcd = serde_json::to_writer(tsunid, &string1).unwrap();
// //     // println!("File converted and written to tsunid.");
// // }

// // fn print_type_of<T>(_: &T) {
// //     println!("{}", std::any::type_name::<T>())
// // }