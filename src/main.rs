use std::fs::{File};
use std::io;
use std::io::{Read, Write};

fn main() {
    let file_path=get_user_input("Enter the file path:");
    match read_file(&file_path) {
        Ok(content)=>{
            println!("Content of the file:");
            println!("{}",content);
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
