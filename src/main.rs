use std::fs::{File};
use std::io;
use std::io::{Read, Write};
use chacha20poly1305::aead::{Aead, OsRng};
use chacha20poly1305::{AeadCore, ChaCha20Poly1305, Key, KeyInit, Nonce};

const AES_BLOCK_SIZE:usize=16;
fn main() {
    let file_path=get_user_input("Enter the file path:");
    match read_file(&file_path) {
        Ok(content)=>{
            println!("Content of the file:");
            println!("{}",content);
            let key = ChaCha20Poly1305::generate_key(&mut OsRng);
            let nonce=ChaCha20Poly1305::generate_nonce(&mut OsRng);
            let encrypted_contect=encrypt(&content,key,nonce);
            println!("Encrypted content:");
            println!("{:?}",encrypted_contect)
        }
        Err(err)=>{
            eprintln!("Error reading the file:{}",err);
        }
    }
}

fn get_user_input(p0: &str) -> String {
    print!("{}",p0);
    io::stdout().flush().unwrap();

    let mut input=String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn read_file(file_path:&str)->io::Result<String>{
    let mut file=File::open(file_path)?;
    let mut content=String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn encrypt(content: &str,key: Key,nonce: Nonce) ->Vec<u8>{
    let cipher=ChaCha20Poly1305::new(&key);
    let ciphertext=cipher.encrypt(&nonce,content.as_ref()).expect("failed to encrypt");
    let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref()).expect("error");
    println!("{:?}",plaintext);

    ciphertext

}

