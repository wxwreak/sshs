# SSH Audits [sshs]

A Lightweight command-line tool written in Rust to audit local SSH keys and files for security weaknesses, weak cryptography, insecure file permissions, and unencrypted private keys.

## Preview
![Preview](https://github.com/wxwreak/sshs/blob/main/preview_sshs.png)

## Features
- **Automated Directory Scanning:** Automatically discovers and inspects files inside the `~/.ssh` directory.
- **Private Key Auditing:** Inspects file permissions to ensure private keys are appropriately restricted, Detects whether private keys are protected with a passphrase, Identifies legacy PEM formats.
- **Public Key Analysis:** Parses public keys and authorized keys to check for cryptographic strengths:

    | Types |
    | :---  | 
    | RSA |
    | DSA |
    | ECDSA |
    | Ed25519 |

---

## Output Legend
- `[+]` **Successful checks.**
- `[*]` **Informational status.**
- `[!]` **Warnings.**
- `[!!]` **Errors.**
- `[!!!]` **Critical.**

## Installation via Makefile
Clone the repo and use the provided Makefile to build and install the binary (`sshs`) directly to `~/.local/bin`:
```bash
git clone https://github.com/wxwreak/sshs
cd sshs
make
```
Available Makefile targets:
- `make build`: Compiles the binary into the `dist/` directory.
- `make install`: Copies the binary to `~/.local/bin`.
- `make all`: Runs both build and install steps.
- `make clean`: Removes the `dist/` build directory.

## Usage
Run the compiled tools directly:
```bash
sshs
```
The tool will automatically locate your `~/.ssh` directory, ignore the configuration and known hosts files, and audits your keys.

## License
This project is licensed under the MIT License. See the [License](LICENSE) file for details.