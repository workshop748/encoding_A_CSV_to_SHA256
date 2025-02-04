use std::fs::File;
use sha2::{Digest,Sha256};
use std::io::{BufRead, BufReader,Write};

fn main()-> std::io::Result<()>
{
    let path1 = "CHANGEME.csv";
    let path2 = "CHANGEME.csv";
    
    let mut output_file =File::create(path2)?;
    //opening the file
    let file = File::open(path1)?;
    let mut reader = BufReader::new(file);
   
    let mut first_line = String::new();
    reader.read_line(&mut first_line)?;
    writeln!(output_file,"{}",first_line)?;

    for line in reader.lines()
    {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() !=2 {
            continue;
        }
        let username = parts[0].trim();
        let passwords = parts[1].trim();

        let mut hasher = Sha256::new();
        hasher.update(passwords.as_bytes());
        let hashed_password = hasher.finalize();

        let hashed_password_hex = format!("{:x}",hashed_password);

        writeln!(output_file, "{},{}",username,hashed_password_hex)?;
    }
    println!("the file has been completed");

    Ok(())
}