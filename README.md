# Passgen

A simple password generator in Rust
## Installation

```bash
  git clone https://github.com/Keegan-JW/passgen
  Move into $PATH
```
    
## Usage

```text
 USAGE:
    passgen [FLAGS] [OPTIONS] 

FLAGS:
    -v, --copy-password
    -h, --help                
    -d, --include-digits
    -c, --include-lowercase
    -s, --include-special
    -u, --include-uppercase
    -V, --version              

OPTIONS:
    -l, --length <length>     [default: 20]
```

## Example

```bash
passgen -l 25 -u -c -d -s
```
