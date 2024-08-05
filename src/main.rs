mod encryption;
mod chunking;
mod metadata;

use encryption::{generate_keys, encrypt, decrypt};
use chunking::chunk_data;
use metadata::Metadata;
use std::fs::File;
use std::io::{Read, Write};
use sha2::{Sha256, Digest};

fn main() {
    // Generate keys
    let (private_key, public_key) = generate_keys();

    // Encrypt data
    let data = "Some data to be stored on decentralized database";
    let bytes: &[u8] = data.as_bytes();
    // Chunk the data
    let chunks = chunk_data(bytes, 64); // chunk size of 64 bytes

    // Encrypt each chunk and store in a simulated distributed storage
    let mut metadata = Metadata::new();

    for chunk in chunks {
        let chunk: &str = std::str::from_utf8(&chunk).unwrap();
        let encrypted_chunk = encrypt(&chunk, &public_key);

        // use the hash of the encrypted chunk as an identifier
        let hash = base64::encode(&encrypted_chunk); 

        let storage_dir = "storage";

    // Ensure the storage directory exists
    if let Err(e) = std::fs::create_dir_all(storage_dir) {
        eprintln!("Failed to create storage directory: {}", e);
        return;
    }

    let hash1 = Sha256::digest(&hash.as_bytes());
    metadata.add_chunk_hash(hash.clone());
    let hash_hex = format!("{:x}", hash1);
    let file_path = format!("{}/{}", storage_dir, hash_hex);
    println!("=======HASH_HEX ====={hash_hex} , ======FILE_PATH====={file_path}");
    

    let mut file = match File::create(&file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to create file: {}", e);
            return;
        },
    };

    // Write some data to the file (optional)
    let bytes: &[u8] = encrypted_chunk.as_bytes();
    if let Err(e) = file.write_all(&bytes) {
        eprintln!("Failed to write to file: {}", e);
    }
    }

    // Store metadata
    let metadata_json = serde_json::to_string(&metadata).expect("failed to serialize metadata");
    let mut file = File::create("metadata.json").expect("failed to create metadata file");
    file.write_all(metadata_json.as_bytes()).expect("failed to write metadata");


     // Simulate retrieval of data
    let metadata: Metadata = serde_json::from_reader(File::open("metadata.json").expect("failed to open metadata file"))
        .expect("failed to deserialize metadata");

    let mut decrypted_data = Vec::new();
    for hash in metadata.chunk_hashes {

        let hash1 = Sha256::digest(&hash.as_bytes());
        let hash_hex = format!("{:x}", hash1);


        let mut file = File::open(format!("storage/{}", hash_hex)).expect("failed to open chunk file");
        let mut encrypted_chunk = Vec::new();
        file.read_to_end(&mut encrypted_chunk).expect("failed to read chunk file");
        let encrypted_chunk: &str = std::str::from_utf8(&encrypted_chunk).unwrap();
        let chunk = decrypt(&encrypted_chunk, &private_key);
        let chunk: &[u8] = chunk.as_bytes();
        decrypted_data.extend_from_slice(&chunk);
    }

    println!("Decrypted Data: {:?}", String::from_utf8(decrypted_data).expect("failed to convert to string"));
}
