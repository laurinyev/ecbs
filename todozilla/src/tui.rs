use std::io::{stdin, stdout, Write};


pub fn prompt_yn(prompt: &str) -> std::io::Result<bool> {
    loop {
        print!("{prompt} [y/n] ");
        let mut string = String::new();
        stdout().flush().unwrap();
        stdin().read_line(&mut string)?;
        
        match string.trim() {
            "y" => return Ok(true),
            "n" => return Ok(false),
            _ => continue
        }
    }
}

pub fn prompt_str_default(prompt: &str, default: &str) -> std::io::Result<String> {
    print!("{prompt} [{default}] ");
    let mut string = String::new();
    stdout().flush().unwrap();
    stdin().read_line(&mut string)?;

    Ok(match string.trim() {
        "" => default.to_string(),
        _ => string.trim().to_string()
    })
}


