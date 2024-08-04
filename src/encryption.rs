extern crate rsa;
extern crate rand;
extern crate base64;

use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, PaddingScheme};
use rand::rngs::OsRng;
use base64::{encode, decode};

pub fn generate_keys() -> (RsaPrivateKey, RsaPublicKey) {
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);
    (private_key, public_key)
}

pub fn encrypt(data: &str, public_key: &RsaPublicKey) -> String {
    let mut rng = OsRng;
    let encrypted_data = public_key.encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), data.as_bytes())
        .expect("failed to encrypt");
    encode(&encrypted_data)
}

pub fn decrypt(encrypted_data: &str, private_key: &RsaPrivateKey) -> String {
    let decrypted_data = private_key.decrypt(PaddingScheme::new_pkcs1v15_encrypt(), &decode(encrypted_data).expect("failed to decode base64"))
        .expect("failed to decrypt");
    String::from_utf8(decrypted_data).expect("failed to convert to string")
}




// extern crate rsa;
// extern crate rand;
// extern crate base64;

// use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, PaddingScheme};
// use rand::rngs::OsRng;
// use base64::{encode, decode};

// pub fn generate_keys() -> (RsaPrivateKey, RsaPublicKey) {
//     let mut rng = OsRng;
//     let bits = 2048;
//     let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
//     let public_key = RsaPublicKey::from(&private_key);
//     (private_key, public_key)
// }

// pub fn encrypt(data: &[u8], public_key: &RsaPublicKey) -> Vec<u8> {
//     let mut rng = OsRng;
//     public_key.encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), data)
//         .expect("failed to encrypt")
// }

// pub fn decrypt(encrypted_data: &[u8], private_key: &RsaPrivateKey) -> Vec<u8> {
//     private_key.decrypt(PaddingScheme::new_pkcs1v15_encrypt(), encrypted_data)
//         .expect("failed to decrypt")
// }


