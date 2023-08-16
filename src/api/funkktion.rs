
// *********************************************************************

// pub fn read_file(tsunid: &Path)->String{

//     let mut file = File::open(tsunid);
//     let mut data = String::new();
//     file.read_to_string(&mut data)?;
//     return data;
// }

// pub fn write_file(tsunid: &Path, _string1: &str) {
//     let mut file = File::create(tsunid).unwrap();
//     file.write_all(_string1.as_bytes()).unwrap();

//     println!("File created successfully")
//     // let mut tsunid = File::create("/home/vhits/Desktop/tsunagu-keygen/tsu-key/tsunid/id.json").expect("Failed to create tsunid");
//     // let abcd = serde_json::to_writer(tsunid, &string1).unwrap();
//     // println!("File converted and written to tsunid.");
// }
// ************************OLD VERSION**********************************
/* pub fn newerr(){
    let tsunid = Path::new("/home/vhits/Desktop/tsunagu-keygen/tsu-key/tsunid/tsun_id.json");
    let tsun_path = tsunid.exists();
    println!("{:?}",tsun_path);
    match tsun_path {
        false => {
            let data = Mnemonic::random(&mut OsRng, Default::default());
            let mnemonic = data.phrase();
            let string1 = String::from_str(mnemonic).unwrap();
            println!("{:?}",string1);
            write_file(tsunid, &string1);
        },
        true => panic!("File banaeli j che bapu"),
    };
    // if tsunid.exists(){
    //     panic!("File already exists!!! ");
    // } else {
        // let data = Mnemonic::random(&mut OsRng, Default::default());
        // let mnemonic = data.phrase();
        // let string1 = String::from_str(mnemonic).unwrap();
        // println!("{:?}",string1);
    //     write_file(tsunid, &string1)
    // }

// ************** Writing file logic ******************
    // let mut tsunid = File::create("/home/vhits/Desktop/tsunagu-keygen/tsu-key/tsunid/id.json").expect("Failed to create tsunid");
    // let abcd = serde_json::to_writer(tsunid, &string1).unwrap();
    // println!("File converted and written to tsunid.");
} */
// ***************************************************************
// pub fn newerr() {
//     let tsunid = Path::new("/home/vhits/Desktop/tsunagu-keygen/tsu-key/tsunid/tsun_id.json");
//     let tsun_path = tsunid.exists();
//     println!("{:?}", tsun_path);
//     match tsun_path {
//         false => {
//             let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);

//             let seed = Seed::new(&mnemonic, "");
//             let secp = Secp256k1::new();
//             let private_key = SecretKey::from_slice(&seed.as_bytes()).expect("Failed to derive private key");
//             // let mut rng = rand::thread_rng();
//             // let mut private_key = SecretKey::new(&secp, &mut rng);
//             // let public_key = PublicKey::from_secret_key(&secp, &private_key);

//             // let private_key_hex = private_key.serialize().encode_hex::<String>();
//             // let public_key_hex = public_key.serialize_uncompressed().encode_hex::<String>();  
//             let public_key = PublicKey::from_secret_key(&secp, &private_key);
//             let public_key_bytes = public_key.serialize_uncompressed();
//             let private_key_bytes = private_key[..].to_vec();

//             let private_key_hex = hex::encode(&private_key_bytes);
//             let public_key_hex = hex::encode(&public_key_bytes);

//             println!("Mnemonic: {}", mnemonic);
//             println!("Private Key: {}", private_key_hex);
//             println!("Public Key: {}", public_key_hex);


//             // #####################chat gpt pehla-7-8-23##################
//             // let account = ExtendedPrivKey::derive(seed.as_bytes(), "m/44'/60'/0'/0/0").unwrap();

//             // let secret_key = SecretKey::parse(&account.secret()).unwrap();
//             // println!("{:?}",secret_key);
//             // let public_key = PublicKey::from_secret_key(&secret_key, &secret_key);
//             // println!("{:?}",public_key);
//             // let s = secret_key.serialize();
//             // println!("{:?}",s);
//             // // let pp = public_key.serialize();
//             // let p = public_key.serialize_uncompressed();
//             // println!("{:?}",p);
//             // let d = s.encode_hex::<String>();
//             // println!("{}",d);
//             // let d1 = p.encode_hex::<String>();
//             // println!("{}",d1);
//             // ############################################################

//             // let data = Mnemonic::random(&mut OsRng, Default::default());
//             // let mnemonic = data.phrase();
//             // let string1 = String::from_str(mnemonic).unwrap();
//             // println!("{:?}",string1);
//             // write_file(tsunid, &string1);
        
//     }
//         true => println!("Keypair already exists"),
//     // if tsunid.exists(){
//     //     panic!("File already exists!!! ");
//     // } else {
//     // let data = Mnemonic::random(&mut OsRng, Default::default());
//     // let mnemonic = data.phrase();
//     // let string1 = String::from_str(mnemonic).unwrap();
//     // println!("{:?}",string1);
//     //     write_file(tsunid, &string1)
//     // }
// }
// }
