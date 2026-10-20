use std::fs;
use std::path::Path;
use colored::*;
mod utils;

fn main() {
    let ascii_art = r#"
   __________ __  __   ___             ___ __      
  / ___/ ___// / / /  /   | __  ______/ (_) /______
  \__ \\__ \/ /_/ /  / /| |/ / / / __  / / __/ ___/         github.com/wxwreak
 ___/ /__/ / __  /  / ___ / /_/ / /_/ / / /_(__  ) 
/____/____/_/ /_/  /_/  |_\__,_/\__,_/_/\__/____/  
                                                   
    "#;
    println!("{}", ascii_art);
    
    if let Some(home_dir) = std::env::var_os("HOME") {
        let ssh_dir = Path::new(&home_dir).join(".ssh");

        if ssh_dir.is_dir() {
            println!("{} Scanning directory: {:?}", "[*]".cyan(), ssh_dir);
            
            if let Ok(entries) = fs::read_dir(&ssh_dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        
                        if !path.is_file() { continue; }
                        let filename = path.file_name().unwrap_or_default().to_string_lossy();

                        if filename == "known_hosts" || filename == "known_hosts.old" || filename == "config" {
                            continue;
                        }

                        println!("\n{} Analyzing: {}", "[*]".cyan(), filename.bold());

                        // Rozdělení: Veřejné klíče vs Privátní klíče (nebo authorized_keys)
                        if filename.ends_with(".pub") || filename == "authorized_keys" {
                            if let Err(e) = utils::audit_public_key(&path) {
                                eprintln!("{} Error auditing public key: {}", "[!!]".red(), e);
                            }
                        } else {
                            if let Ok(content) = fs::read_to_string(&path) {
                                if content.contains("-----BEGIN") {
                                    if let Err(e) = utils::audit_private_key(&path) {
                                        eprintln!("{} Error auditing private key: {}", "[!!]".red(), e);
                                    }
                                } else {
                                    println!("{} Skipping non-key file", "[-]".dimmed());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}