use docx_rs::*;
use rand::{Rng, thread_rng};
use std::io;
use aes_gcm::{Aes256Gcm, Key, Nonce}; // Or use Aes128Gcm
use aes_gcm::aead::{Aead, NewAead};
use base64::{encode, decode};
use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, PaddingScheme};
use rand::rngs::OsRng;

use docx::document::Paragraph;
use docx::DocxFile;

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
    let decoded_nonce = decode(parts[0]).expect("Failed to decode nonce");
    let decoded_ciphertext = decode(parts[1]).expect("Failed to decode ciphertext");

    let nonce = Nonce::from_slice(&decoded_nonce);

    // Decrypt the ciphertext
    let decrypted_data = cipher.decrypt(nonce, decoded_ciphertext.as_ref()).expect("Decryption failure!");
    String::from_utf8(decrypted_data).expect("Failed to convert decrypted data to String")
}
pub fn encrypt_docx(input: String, public_key: &RsaPublicKey) -> Result<(), DocxError> {
    let path = std::path::Path::new("./output.docx");
    let file = std::fs::File::create(&path).unwrap();
    let key: [u8; 32] = *b"an example very very secret key.";
    // Generate a random encryption method
    let mut rng = thread_rng();
    let encryption_method = rng.gen_range(0..2); // for example, choose between symmetric or asymmetric

    // Encrypt the input text using the selected encryption method
    let encrypted_text = match encryption_method {
        0 => encrypt_symmetric(input,&key), // encrypt using symmetric encryption
        1 => encrypt_asymmetric(input,public_key), // encrypt using asymmetric encryption
        _ => panic!("Invalid encryption method") // handle error if invalid method selected
    };

    // Add the encrypted text to a new paragraph
    Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text(encrypted_text)))
        .build()
        .pack(file)?;

    Ok(())
}



fn encrypt_asymmetric(input: String, public_key: &RsaPublicKey) -> String {
    let padding = PaddingScheme::new_pkcs1v15_encrypt();

    let mut rng = OsRng;
    let encrypted_data = public_key.encrypt(&mut rng, padding, input.as_bytes()).expect("Encryption failed");

    // Encode the encrypted data as a base64 string
    encode(&encrypted_data)
}

pub fn decrypt_docx(encryption_method: u8, password: String, private_key: &RsaPrivateKey) -> Result<(), DocxError> {
    let path = std::path::Path::new("./output.docx");

    let docx = DocxFile::from_file(&path).unwrap();
    let mut docx = docx.parse().unwrap();

    let key: [u8; 32] = *b"an example very very secret key.";
    // Read the encrypted text from the docx file
    let encrypted_text = docx.paragraphs()[0].runs()[0].text();

    // Use the selected encryption method and the decryption password to decrypt the text
    let decrypted_text = match encryption_method {
        0 => decrypt_symmetric(encrypted_text, &key), // decrypt using symmetric encryption
        1 => decrypt_asymmetric(encrypted_text, private_key), // decrypt using asymmetric encryption
        _ => panic!("Invalid encryption method") // handle error if invalid method selected
    };

    // Print or display the decrypted text to the user
    println!("Decrypted text: {}", decrypted_text);

    Ok(())
}


fn decrypt_asymmetric(encrypted_text: String, private_key: &RsaPrivateKey) -> String {
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let decoded_encrypted_text = decode(&encrypted_text).expect("Failed to decode encrypted text");

    let decrypted_data = private_key.decrypt(padding, &decoded_encrypted_text).expect("Decryption failed");

    String::from_utf8(decrypted_data).expect("Failed to convert decrypted data to String")
}
fn prompt_password() -> String {
    println!("Please enter the decryption password:");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read line");
    password.trim().to_string()
}