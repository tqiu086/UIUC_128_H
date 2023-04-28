use rsa::RsaPrivateKey;
use std::io::Error;

fn main() -> Result<(), Error> {
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
    encrypt_docx(input, &public_key)?;

    // Ask the user which encryption method they used (symmetric or asymmetric)
    println!("Please enter the encryption method you used (0 for symmetric, 1 for asymmetric):");
    let mut method_input = String::new();
    io::stdin().read_line(&mut method_input)?;
    let encryption_method: u8 = method_input.trim().parse().expect("Invalid input for encryption method");

    // Decrypt the encrypted text from the docx file using the selected method
    match encryption_method {
        0 => {
            // Prompt the user for the decryption password
            let password = prompt_password();
            decrypt_docx(encryption_method, password, &private_key)?;
        }
        1 => {
            decrypt_docx(encryption_method, String::new(), &private_key)?;
        }
        _ => panic!("Invalid encryption method"),
    }

    Ok(())
}