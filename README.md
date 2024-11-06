# What is?
rsdump is a basic hexdump tool written in Rust.

# How to use?
```
rsdump [options] path
```
## Options
- `--ascii`: prints the ascii representation of each byte
- `--author`: prints authorship information
- `--help`: prints help information

# Compiling
The compilation process can be achieved using cargo
```
cargo build
```
# Outro
The [[test/]] folder contains some files used to test the application. It includes two text files, a plain one and a C one, and a C compiled binary.
