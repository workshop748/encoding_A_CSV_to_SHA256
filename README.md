# Encoding a CSV to SHA-256  

## Overview  
For a school project, I was tasked with creating a client-server model login page.  
To enhance security, I created a program that reads usernames and passwords from a CSV file and applies **SHA-256 hashing** to the passwords before storing them in a new file.  

## Features  
- Reads usernames and passwords from a CSV file  
- Skips the first line (header)  
- Hashes passwords using **SHA-256**  
- Saves the results in a new CSV file (`username,hashed_password`)  

## Installation  
### **Requirements**  
- Rust installed ([Download Rust](https://www.rust-lang.org/tools/install))  
- `sha2` crate for hashing  

### **Setup**  
1. Clone this repository:  
   ```sh
   git clone https://github.com/yourusername/encoding_A_CSV_to_SHA2.git
   cd encoding_A_CSV_to_SHA2
