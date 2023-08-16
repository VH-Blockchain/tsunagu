//     // let example_msg = b"Hello, world!";
//     // let signature: Signature = signing_key.sign(example_msg);
//     // assert!(verification_key.verify(example_msg, &signature).is_ok());
// }
use bip39::{Language, Mnemonic, MnemonicType, Seed};
use k256::elliptic_curve::sec1::ToEncodedPoint;
use secp256k1::{PublicKey, SecretKey};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use tiny_hderive::bip32::ExtendedPrivKey;

// use rand_core::OsRng;
// pub fn newerror() {
//     // let secretkey = eth_keystore::new("/home/vhtushar/CLI_WAllET/wallet",&mut thread_rng(),"",None);
//     // let wallet = LocalWallet::new_keystore("/home/vhtushar/CLI_WAllET/wallet",&mut thread_rng(),"",None);
//     // println!("Hello qoels {:?},{:?}",wallet.expect("ss"),secretkey);
//     let data = generate();
//     println!("{}", data);
//     let data1 = generate_using_passphrase(String::from("1234"));
//     println!("{}", data1);
//     // for i in 0..5 {
//     //     let mnemonic = String::from("east trust vendor essence retire merit observe inform search sniff ordinary embark");
//     //     let path =format!("m/44'/60'/0'/0/{i}");
//     //     let data2 = generate_keypair_forspecificindex(mnemonic, path, String::from("1234"));
//     //     println!("{}", data2);
//     // }
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct Keypair {
    pub mnemonics: String,
    pub private_key: String,
    pub public_key: String,
    pub public_address: String,
    pub path: String,
}

// impl std::fmt::Display for Keypair {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "Mnemonic: {},\nPrivate_Key: {},\nPublic_Key: {},\nPublic_Address: {},\nPath: {}\n",
//             self.mnemonic,
//             hex::encode(self.private_key),
//             hex::encode(self.public_key),
//             self.public_address,
//             self.path
//         )
//     }
// }

pub fn generate() -> Keypair {
    generate_using_passphrase(String::from(""))
}
pub fn generate_from_mnemonic(mnemonic: String,_passprase: String) -> Keypair{
    generate_keypair_forspecificindex(mnemonic,"m/44'/60'/0'/0/0".to_string(),_passprase)
} 
pub fn generate_using_passphrase(_passprase: String) -> Keypair {
    let mnemonics = Mnemonic::new(MnemonicType::Words12, Language::English);
    generate_keypair_forspecificindex(
        mnemonics.into_phrase(),
        "m/44'/60'/0'/0/0".to_string(),
        _passprase,
    )
}
pub fn generate_keypair_forspecificindex(
    mnemonic: String,
    path: String,
    _passprase: String,
) -> Keypair {
    let mnemonics = Mnemonic::from_phrase(&mnemonic.as_str(), Language::English).unwrap();
    let seed = Seed::new(&mnemonics, &_passprase);
    let account = ExtendedPrivKey::derive(seed.as_bytes(), path.as_str()).unwrap();
    let secret_key = SecretKey::parse(&account.secret()).unwrap();
    let public_key = PublicKey::from_secret_key(&secret_key);
    let priv_key: [u8; 32] = secret_key.serialize();
    let private_key = hex::encode(priv_key);
    let pub_key = public_key.serialize_compressed();
    let public_key = hex::encode(pub_key);
    let pubkeyy = k256::PublicKey::from_sec1_bytes(&pub_key).expect("failed parsing eth bytes");
    let addr = Keccak256::digest(&pubkeyy.to_encoded_point(false).as_bytes()[1..]);
    let public_address = eip55::checksum(&hex::encode(&addr[12..]));
    // println!("Public Address is {}", public_address);
    let mnemonics = mnemonics.into_phrase();
    let keypair = Keypair {
        mnemonics,
        public_address,
        private_key,
        public_key,
        path,
    };
    return keypair;
}