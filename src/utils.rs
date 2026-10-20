// src/utils.rs
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::os::unix::fs::PermissionsExt;
use ssh_key::{PublicKey, PrivateKey};
use colored::*;

pub fn audit_public_key<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        let line_trim = line.trim();

        if line_trim.is_empty() || line_trim.starts_with('#') {
            continue;
        }
        
        match PublicKey::from_openssh(line_trim) {
            Ok(key) => {
                
                let algo_str = key.algorithm().to_string();

                if algo_str.contains("ed25519") {
                    println!("{} Algorithm Ed25519 (Secure)", "[+]".green());
                } else if algo_str.contains("rsa") {
                    println!("{} Algorithm RSA (Legacy format, secure only if 3072b+)", "[!]".yellow());
                } else if algo_str.contains("dsa") {
                    println!("{} Algorithm DSA (Deprecated)", "[!!!]".red());
                } else {
                    println!("{} Algorithm: {} (Check security)", "[!]".yellow(), algo_str);
                }
            }
            Err(e) => {
                eprintln!("{} Line: {}: Failed to parse keys [{}]", "[!!]".red(), index + 1, e);
            }
        }
    }
    Ok(())
}

pub fn audit_private_key<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    let path_ref = path.as_ref();

    if let Ok(metadata) = fs::metadata(path_ref) {
        let mode = metadata.permissions().mode() & 0o777;
        if mode != 0o600 && mode != 0o400 {
            println!("{} Insecure permissions: ({:o}), set permissions to 600", "[!!!]".red(), mode);
        } else {
            println!("{} File permissions are secure: ({:o})", "[+]".green(), mode);
        }
    }

    let content = match fs::read_to_string(path_ref) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{} Failed to read file: {}", "[!!]".red(), e);
            return Err(e);
        }
    };
    if content.contains("-----BEGIN RSA PRIVATE KEY-----") {
        println!("{} Legacy PEM format detected, convert it via ssh-keygen", "[!]".yellow());
    }

    match PrivateKey::from_openssh(&content) {
        Ok(pkey) => {
            println!("{} Private key is not encrypted with a passphrase", "[!!!]".red());
            
            let algo_str = pkey.algorithm().to_string();
            
            if algo_str.contains("ed25519") {
                println!("{} Algorithm Ed25519 (Secure)", "[+]".green());
            } else if algo_str.contains("rsa") {
                println!("{} Algorithm RSA (Legacy format, secure only if 3072b+)", "[!]".yellow());
            } else if algo_str.contains("dsa") {
                println!("{} Algorithm DSA (Deprecated)", "[!!!]".red());
            } else {
                println!("{} Algorithm: {} (Check security)", "[!]".yellow(), algo_str);
            }
        }
        Err(ssh_key::Error::Encrypted) => {
            println!("{} Private key is secure (using a passphrase)", "[+]".green());
        }
        Err(e) => {
            println!("{} Failed to parse private key: {}", "[!!]".red(), e);
        }
    }
    Ok(())
}