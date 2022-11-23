use bip32::{Mnemonic, Prefix, XPrv};
use bitcoin::hashes::Hash;
use bitcoin::hashes::{hmac, sha512, HashEngine};
use bitcoin::util::bip32::ChildNumber;
use bitcoin::util::bip32::DerivationPath;
use bitcoin::util::bip32::ExtendedPrivKey;
use clap::Parser;
use rand_core::OsRng;
use std::str::FromStr;

#[derive(Parser)]
#[command( version, long_about = None)]

// #[clap(about, version, author)]
struct Args {
    xprv: String,

    #[clap(short = 'L', long, default_value = "21")]
    pwd_len: u32,

    #[clap(short = 'I', long, default_value = "0")]
    index: u32,
}

fn main() {
    let args = Args::parse();

    println!("\n");
    println!("    ________________________            ______   ________ __________                                                ___   ________                                   __                ");
    println!("    \\______   \\   \\______   \\          /  __  \\ |   ____/ \\______   \\_____    ______ ________  _  _____________  __| _/  /  _____/  ____   ____   ________________ _/  |_  ___________ ");
    println!("     |    |  _/   ||     ___/  ______  >      < |____  \\   |     ___/\\__  \\  /  ___//  ___/\\ \\/ \\/ /  _ \\_  __ \\/ __ |  /   \\  ____/ __ \\ /    \\_/ __ \\_  __ \\__  \\\\   __\\/  _ \\_  __ \\");
    println!("     |    |   \\   ||    |     /_____/ /   --   \\/       \\  |    |     / __ \\_\\___ \\ \\___ \\  \\     (  <_> )  | \\/ /_/ |  \\    \\_\\  \\  ___/|   |  \\  ___/|  | \\// __ \\|  | (  <_> )  | \\/");
    println!("     |______  /___||____|             \\________/________/  |____|    (____  /____  >____  >  \\/\\_/ \\____/|__|  \\____ |   \\______  /\\___  >___|  /\\___  >__|  (____  /__|  \\____/|__|   ");
    println!("            \\/                                                            \\/     \\/     \\/                          \\/          \\/     \\/     \\/     \\/           \\/                   ");
    println!("\n");
    println!("     https://github.com/montanasnow/BIP-85-Password-generator");
    println!("     Derives a deterministic password from the key nor mnemonic phrase based on BIP-85");
    println!("     Please provide a BIP-32 root key (xprv...) or a BIP-39 mnemonic.");
    println!("     Options: -L for password length (default 21) and -I index (default 0)");
    println!("     Based on https://coldcard.com/docs/bip85-passwords and https://github.com/scgbckbone/bips/blob/passwords/bip-0085.mediawiki#PWD and https://github.com/scgbckbone/btc-hd-wallet/blob/master/tests/test_bip85.py");
    println!("     The software is provided "AS-IS!", without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, ");
    println!("     fitness for a particular purpose and noninfringement. In no event shall the authors or copyright holders be liable for any claim, damages or other liability,");
    println!("     whether in an action of contract, tort or otherwise, arising from, out of or in connection with the software or the use or other dealings in the software. ");
    println!("     Not for use in Europe or socialist countries");
    println!("\n");

    let root_xprv = if !args.xprv.is_empty() && args.xprv.starts_with("xprv") {
        println!("     XPRV:            {}", &args.xprv);
        ExtendedPrivKey::from_str(&*args.xprv).unwrap()
        println!("Enter your name :");
        let b1 = std::io::stdin().read_line(&mut line).unwrap();
    } else if !args.xprv.is_empty() {
        println!("     Mnemonic:        {}", &args.xprv);
        let mnemonic = Mnemonic::new(&args.xprv, Default::default()).unwrap();
        let seed = mnemonic.to_seed("");
        ExtendedPrivKey::from_str(&XPrv::new(&seed).unwrap().to_string(Prefix::XPRV)).unwrap()
    } else {
        let mnemonic = Mnemonic::random(&mut OsRng, Default::default());
        let seed = mnemonic.to_seed("");
        ExtendedPrivKey::from_str(&XPrv::new(&seed).unwrap().to_string(Prefix::XPRV)).unwrap()
    };

    println!("     Password Length: {:?}", args.pwd_len);
    println!("     Index:           {:?}", args.index);

    let secp = bitcoin::secp256k1::Secp256k1::new();

    let path = DerivationPath::from(vec![
        ChildNumber::Hardened { index: 707764 },
        ChildNumber::from_hardened_idx(args.pwd_len).unwrap(),
        ChildNumber::from_hardened_idx(args.index).unwrap(),
    ]);

    const BIP85_CHILD_NUMBER: ChildNumber = ChildNumber::Hardened { index: 83696968 };
    let bip85_root = root_xprv.ckd_priv(&secp, BIP85_CHILD_NUMBER).unwrap();
    let derived = bip85_root.derive_priv(&secp, &path).unwrap();
    let mut h = hmac::HmacEngine::<sha512::Hash>::new("bip-entropy-from-k".as_bytes());
    h.input(&derived.private_key.to_bytes());
    let data = hmac::Hmac::from_engine(h).into_inner().to_vec();

    let entropy_b64 = base64::encode(&data[0..64]);
    let password = entropy_b64[0..args.pwd_len as usize].to_string();

    println!("\n");
    println!("Password: {}", &password);
    println!("\n");
}
