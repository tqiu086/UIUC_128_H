use std::io::{self, Error};
use rand::rngs::OsRng;
use rand::{Rng, thread_rng};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use base64::{encode, decode};
use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, PaddingScheme};


use docx_rs::*;
use docx_rust::DocxFile;

fn decrypt_asymmetric(encrypted_text: String, private_key: &RsaPrivateKey) -> String {
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let decoded_encrypted_text = decode(&encrypted_text).expect("Failed to decode encrypted text");

    let decrypted_data = private_key.decrypt(padding, &decoded_encrypted_text).expect("Decryption failed");

    String::from_utf8(decrypted_data).expect("Failed to convert decrypted data to String")
}

fn encrypt_symmetric(input: String, key: &[u8; 32]) -> String {
    let cipher = Aes256Gcm::new(Key::from_slice(key));

    // Use a unique nonce (number used once) for each encryption
    let mut rng = thread_rng();
    let mut nonce = [0u8; 12];
    rng.fill(&mut nonce);
    let nonce = Nonce::from_slice(&nonce);

    // Encrypt the input text
    let ciphertext = cipher.encrypt(nonce, input.as_bytes().as_ref()).expect("Encryption failure!");

    // Encode the nonce and ciphertext as base64 strings and concatenate them
    let encoded_nonce = encode(&nonce);
    let encoded_ciphertext = encode(&ciphertext);

    format!("{}|{}", encoded_nonce, encoded_ciphertext)
}

fn decrypt_symmetric(encrypted_text: String, key: &[u8; 32]) -> String {
    let cipher = Aes256Gcm::new(Key::from_slice(key));

    // Split the encrypted text into nonce and ciphertext
    let parts: Vec<&str> = encrypted_text.split('|').collect();
    
    if parts.len() != 2 {
        panic!("Invalid encrypted text format");
    }

    let decoded_nonce = decode(parts[0]).expect("Failed to decode nonce");
    let decoded_ciphertext = decode(parts[1]).expect("Failed to decode ciphertext");

    let nonce = Nonce::from_slice(&decoded_nonce);

    // Decrypt the ciphertext
    let decrypted_data = cipher.decrypt(nonce, decoded_ciphertext.as_ref()).expect("Decryption failure!");
    String::from_utf8(decrypted_data).expect("Failed to convert decrypted data to String")
}


fn encrypt_asymmetric(input: String, public_key: &RsaPublicKey) -> String {
    let padding = PaddingScheme::new_pkcs1v15_encrypt();

    let mut rng = OsRng;
    let encrypted_data = public_key.encrypt(&mut rng, padding, input.as_bytes()).expect("Encryption failed");

    // Encode the encrypted data as a base64 string
    encode(&encrypted_data)
}

pub fn encrypt_docx(input: String, public_key: &RsaPublicKey) -> Result<[u8; 32], io::Error> {
    let path = std::path::Path::new("./output.docx");
    let file = std::fs::File::create(&path).unwrap();
    let mut key: [u8; 32] = [0; 32];
    let mut rng = thread_rng();
    rng.fill(&mut key);
    let mut rng = thread_rng();
    let encryption_method = rng.gen_range(0..2);

    let encrypted_text = match encryption_method {
        0 => encrypt_symmetric(input, &key),
        1 => encrypt_asymmetric(input, public_key),
        _ => panic!("Invalid encryption method")
    };

    docx_rs::Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text(encrypted_text)))
        .build()
        .pack(file)?;

    Ok(key)
}

pub fn decrypt_docx(encryption_method: u8, private_key: &RsaPrivateKey, secret_key: &[u8; 32]) -> Result<(), io::Error> {
    let path = std::path::Path::new("./output.docx");
    let docx = DocxFile::from_file(&path).unwrap();
    let mut docx = docx.parse().unwrap();

    let encrypted_text = docx.document.body.text();

    let decrypted_text = match encryption_method {
        0 => decrypt_symmetric(encrypted_text, secret_key),
        1 => decrypt_asymmetric(encrypted_text, private_key),
        _ => panic!("Invalid encryption method")
    };

    println!("Decrypted text: {}", decrypted_text);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to the docx encryption program!");

    // Generate an RSA key pair
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("Failed to generate private key");
    let public_key = private_key.to_public_key();

    // Prompt the user for the input text
    println!("Please enter the text you want to encrypt:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input = input.trim().to_string();

    // Encrypt the input text and save it to a new docx file
    let key = encrypt_docx(input, &public_key)?;

    // Ask the user which encryption method they used (symmetric or asymmetric)
    println!("Please enter the encryption method you used (0 for symmetric, 1 for asymmetric):");
    let mut method_input = String::new();
    io::stdin().read_line(&mut method_input)?;
    let encryption_method: u8 = method_input.trim().parse().expect("Invalid input for encryption method");

    // Decrypt the encrypted text from the docx file using the selected method
    match encryption_method {
        0 => {
            println!("Secret key used for symmetric encryption: {:?}", key);
            decrypt_docx(encryption_method, &private_key, &key)?;
        }
        1 => {
            println!("No password or secret key required for asymmetric encryption.");
            decrypt_docx(encryption_method, &private_key, &[0; 32])?;
        }
        _ => panic!("Invalid encryption method"),
    }

    Ok(())
}