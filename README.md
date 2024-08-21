# Passgen

A simple password generator in Rust
## Installation

```bash
git clone https://github.com/Keegan-JW/passgen
cd passgen
cargo build --release
# Move the binary into $PATH
```
    
## Usage

```text
Generate random passwords

Usage: passgen [OPTIONS]

Options:
  -l, --length <LENGTH>    The length of the password [default: 20]
  -u, --include-uppercase
  -c, --include-lowercase
  -d, --include-digits
  -s, --include-special
  -v, --copy-password
  -h, --help               Print help
  -V, --version            Print version
```

## Example

```bash
passgen -l 25 -u -c -d -s
# Or
passgen -ucdsl 25
```
