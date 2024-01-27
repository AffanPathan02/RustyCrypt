use std::fs::{File};
use std::io;
use std::io::{Read, Write};

fn main() {
    let file_path=get_user_input("Enter the file path:");
    match read_file(&file_path) {
        Ok(content)=>{
            println!("Content of the file:");
            println!("{}",content);
            let operation= get_user_input("Enter encrypt or decrypt :");
            match operation.to_lowercase().as_str(){
                "encrypt"=>{
                    let encrypted_contect=encrypt(&content);
                    println!("Encrypted content:");
                    println!("{}",encrypted_contect)
                },
                "decrypt"=>{
                    let decrypted_content=decrypt(&content);
                    println!("decrypted content:");
                    println!("{}",decrypted_content)
                },
                _=>{
                    eprintln!("Invalid operation. Use 'encrypt', 'decrypt', or 'none'.");
                    return;
                }
            }
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

fn encrypt(content: &str)->String{
    let encrypted_content:String=content
        .chars()
        .map(|c|{
            if c.is_ascii_alphabetic() {
                let base=if c.is_ascii_lowercase(){b'a'}else { b'A' };
                (((c as u8 - base + 3) % 26) + base) as char
            }else{
                c
            }
        })
        .collect();
    encrypted_content
}

fn decrypt(content: &str) -> String {
    let decrypted_content: String = content
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                (((c as u8 - base + 23) % 26) + base) as char
            } else {
                c
            }
        })
        .collect();
    decrypted_content
}