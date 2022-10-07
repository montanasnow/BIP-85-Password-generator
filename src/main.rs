// use base64::encode;
use bitcoin::hashes::{hmac, sha512, HashEngine};
use bitcoin::secp256k1::{self, Secp256k1};
// use bitcoin::secp256k1::{Secp256k1, SecretKey};

// use bitcoin::hashes::{hmac, sha512, HashEngine};
// use bitcoin::bitcoin_hashes::Hash;
use bitcoin::hashes::Hash;
use bitcoin::util::bip32::ChildNumber;
use bitcoin::util::bip32::DerivationPath;
use bitcoin::util::bip32::ExtendedPrivKey;
use clap::Parser;
// use hmac_sha512::HMAC;
use std::str::FromStr;

#[derive(Parser)]
#[command( version, long_about = None)]

// #[clap(about, version, author)]
struct Args {
    // #[clap(long)]
    xprv: String,

    #[clap(short = 'L', long, default_value = "21")]
    pwd_len: u32,

    #[clap(short = 'I', long, default_value = "0")]
    index: u32,
}

fn main() {
    let args = Args::parse();

    println!("\n");
    println!("    __________._____________            ______   ________    __________                                                ___                                               __                ");
    println!("    \\______   \\   \\______   \\          /  __  \\ |   ____/    \\______   \\_____    ______ ________  _  _____________  __| _/       ____   ____   ____   ________________ _/  |_  ___________ ");
    println!("     |    |  _/   ||     ___/  ______  >      < |____  \\      |     ___/\\__  \\  /  ___//  ___/\\ \\/ \\/ /  _ \\_  __ \\/ __ |       / ___\\_/ __ \\ /    \\_/ __ \\_  __ \\__  \\\\   __\\/  _ \\_  __ \\");
    println!("     |    |   \\   ||    |     /_____/ /   --   \\/       \\     |    |     / __ \\_\\___ \\ \\___ \\  \\     (  <_> )  | \\/ /_/ |      / /_/  >  ___/|   |  \\  ___/|  | \\// __ \\|  | (  <_> )  | \\/");
    println!("     |______  /___||____|             \\______  /______  /     |____|    (____  /____  >____  >  \\/\\_/ \\____/|__|  \\____ |      \\___  / \\___  >___|  /\\___  >__|  (____  /__|  \\____/|__|   ");
    println!("            \\/                               \\/       \\/                     \\/     \\/     \\/                          \\/     /_____/      \\/     \\/     \\/           \\/                   ");

    // println!("xpriv {:?}", args.xprv);
    println!("pwd_len {:?}", args.pwd_len);
    println!("index {:?}", args.index);

    let secp = Secp256k1::new();
    let root = ExtendedPrivKey::from_str(&*args.xprv).unwrap();

    let path = DerivationPath::from(vec![
        ChildNumber::Hardened { index: 707764 },
        ChildNumber::from_hardened_idx(args.pwd_len).unwrap(),
        ChildNumber::from_hardened_idx(args.index).unwrap(),
    ]);

    let data = derive(&secp, &root, &path);

    let entropyb = base64::encode(&data[0..64]);
    let password = entropyb[0..args.pwd_len as usize].to_string();
    println!("\npassword: {}", &password);
    println!("\n");
}

fn derive<C: secp256k1::Signing, P: AsRef<[ChildNumber]>>(
    secp: &Secp256k1<C>,
    root: &ExtendedPrivKey,
    path: &P,
) -> Vec<u8> {
    const BIP85_CHILD_NUMBER: ChildNumber = ChildNumber::Hardened { index: 83696968 };
    let bip85_root = root.ckd_priv(&secp, BIP85_CHILD_NUMBER).unwrap();
    let derived = bip85_root.derive_priv(&secp, &path).unwrap();
    let mut h = hmac::HmacEngine::<sha512::Hash>::new("bip-entropy-from-k".as_bytes());
    h.input(&derived.private_key.to_bytes());
    let data = hmac::Hmac::from_engine(h).into_inner();
    data.to_vec()
}
