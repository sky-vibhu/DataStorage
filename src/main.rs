mod encryption;
mod blockchain;

use encryption::{generate_keys, encrypt, decrypt};
use blockchain::Blockchain;

fn main() {
    // Generate keys
    let (private_key, public_key) = generate_keys();
    // println!("Public key: {:?}", public_key);
    // println!("Private key: {:?}", private_key);

    // Encrypt data
    let data = "Sensitive Data";
    let encrypted_data = encrypt(data, &public_key);
    println!("Encrypted Data: {}", encrypted_data);

    // Store encrypted data on the blockchain
    let mut blockchain = Blockchain::new();
    blockchain.add_block(encrypted_data.clone());
    
    // Retrieve and decrypt data
    if let Some(block) = blockchain.get_block(0) {
        let decrypted_data = decrypt(&block.data, &private_key);
        println!("Decrypted Data: {}", decrypted_data);
    } else {
        println!("No block found");
    }
}



// mod encryption;
// mod chunking;
// mod metadata;

// use encryption::{generate_keys, encrypt, decrypt};
// use chunking::chunk_data;
// use metadata::Metadata;
// use serde_json;
// use std::fs::{self, File} ;
// use std::io::{self, Read, Write};

// // fn main() {
// //     // Generate RSA keys
// //     let (private_key, public_key) = generate_keys();

// //     // Sample large data
// //     let data = b"Some large data to be stored in the decentralized database...";
    
// //     // Chunk the data
// //     let chunks = chunk_data(data, 64); // chunk size of 64 bytes

// //     // Encrypt each chunk and store in a simulated distributed storage
// //     let mut metadata = Metadata::new();
// //     for chunk in chunks {
// //         let encrypted_chunk = encrypt(&chunk, &public_key);
// //         let hash = base64::encode(&encrypted_chunk); // use the hash of the encrypted chunk as an identifier
// //         metadata.add_chunk_hash(hash.clone());
// //         // Simulate storing the chunk in distributed storage
// //         let mut file = File::create(format!("storage/{}", hash)).expect("failed to create file");
// //         file.write_all(&encrypted_chunk).expect("failed to write to file");
// //     }

// //     // Store metadata
// //     let metadata_json = serde_json::to_string(&metadata).expect("failed to serialize metadata");
// //     let mut file = File::create("metadata.json").expect("failed to create metadata file");
// //     file.write_all(metadata_json.as_bytes()).expect("failed to write metadata");

// //     // Simulate retrieval of data
// //     let metadata: Metadata = serde_json::from_reader(File::open("metadata.json").expect("failed to open metadata file"))
// //         .expect("failed to deserialize metadata");

// //     let mut decrypted_data = Vec::new();
// //     for hash in metadata.chunk_hashes {
// //         let mut file = File::open(format!("storage/{}", hash)).expect("failed to open chunk file");
// //         let mut encrypted_chunk = Vec::new();
// //         file.read_to_end(&mut encrypted_chunk).expect("failed to read chunk file");
// //         let chunk = decrypt(&encrypted_chunk, &private_key);
// //         decrypted_data.extend_from_slice(&chunk);
// //     }

// //     println!("Decrypted Data: {:?}", String::from_utf8(decrypted_data).expect("failed to convert to string"));
// // }






// #[tokio::main]
// async fn main() -> io::Result<()> {
//     // Generate RSA keys
//     let (private_key, public_key) = generate_keys();

//     // Sample large data
//     let data = b"Some large data to be stored in the decentralized database...";
    
//     // Chunk the data
//     let chunks = chunk_data(data, 64); // chunk size of 64 bytes

//     // Encrypt each chunk and store in a simulated distributed storage
//     let mut metadata = Metadata::new();
//     let client = Client::new();
//     for chunk in chunks {
//         let encrypted_chunk = encrypt(&chunk, &public_key);
//         let hash = base64::encode(&encrypted_chunk); // use the hash of the encrypted chunk as an identifier
//         metadata.add_chunk_hash(hash.clone());
        
//         // Store the chunk in distributed storage
//         let res = client.post("http://127.0.0.1:3030/store")
//             .json(&encrypted_chunk)
//             .send()
//             .await
//             .expect("failed to store chunk");
//         assert_eq!(res.status(), 200);
//     }

//     // Store metadata
//     let metadata_json = serde_json::to_string(&metadata).expect("failed to serialize metadata");
    
//     let path = "path/to/directory/metadata.json";
    
//     // Extract the directory from the path
//     if let Some(parent) = std::path::Path::new(path).parent() {
//         // Create the directory if it does not exist
//         if !parent.exists() {
//             fs::create_dir_all(parent).expect("failed to create directory");
//         }
//     }

//     // Create the file
//     let mut file = File::create(path).expect("failed to create file");
//     file.write_all(metadata_json.as_bytes()).await.expect("failed to write metadata");

//     // Simulate retrieval of data
//     let mut file = File::open(path).await.expect("failed to open metadata file");
//     let mut metadata_json = Vec::new();
//     file.read_to_end(&mut metadata_json).await.expect("failed to read metadata file");
//     let metadata: Metadata = serde_json::from_slice(&metadata_json).expect("failed to deserialize metadata");

//     let mut decrypted_data = Vec::new();
//     for hash in metadata.chunk_hashes {
//         let res = client.get(format!("http://127.0.0.1:3030/retrieve/{}", hash))
//             .send()
//             .await
//             .expect("failed to retrieve chunk");
//         assert_eq!(res.status(), 200);
//         let encrypted_chunk: Vec<u8> = res.json().await.expect("failed to parse chunk");
//         let chunk = decrypt(&encrypted_chunk, &private_key);
//         decrypted_data.extend_from_slice(&chunk);
//     }

//     println!("Decrypted Data: {:?}", String::from_utf8(decrypted_data).expect("failed to convert to string"));
//     Ok(())
// }
