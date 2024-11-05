// rsdump a basic hexdump made in Rust 
// Copyright (C) 2024 Caio Silva Couto
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE 
// SOFTWARE.

#![allow(unused_variables)]
#![allow(dead_code)]

use std::env::args;
use std::path::Path;
use std::fs;
use std::io::Error;

const HELP_MESSAGE:&str = "Usage: rsdump [options] path
Basic hexdump tool written in Rust
Options:
    --ascii: prints ascii values table
    --help: prints this help message
    --author: prints information about authorship";

const AUTHORSHIP:&str = "Author: Caio Silva Couto
Contact: 
    GitHub: th3worst4
    email: caiocouto25@hotmail.com
License:
    This piece of software is under the MIT license";

fn read_raw_data(path:String) -> Result<Vec<u8>, Error> { 
    let contents = fs::read(path)?;
    Ok(contents)
}

fn create_char_vector(vector:Vec<u8>) -> Result<Vec<char>, Error> {
    let mut result = Vec::new();
    for i in vector.iter() {
        if *i < 0x21 {
            result.push('.')
        }else {
            result.push(*i as char)
        }
    }
    Ok(result)
}

fn print_vector<T: std::fmt::Display>(vector:Vec<T>) {
    for i in vector.iter() {
        println!("{}", i);
    }
}

fn print_formated(raw_data:Vec<u8>, ascii_vector:Vec<char>, print_char:bool){
    let mut ascii_string = String::new();
    for (index, data) in raw_data.iter().enumerate() {
        if index % 16 == 0 {
            if index != 0 {
                if print_char {
                    print!("|{}|", ascii_string);
                    ascii_string = String::new();
                }
                print!("\n");
            }
            print!("{:08x} ", index / 16);
        }
        print!("{:02x}", data);
        ascii_string.push(ascii_vector[index]);
        if index % 16 == 7 {
            print!("  ");
        }else {
            print!(" ");
        }
        if index + 1 == raw_data.len(){
            let missing_elements = 16 - (index % 16) - 1;
            for i in 0..missing_elements {
                print!("  ");
                if index % 16 == 7 {
                    print!("  ");
                }else {
                    print!(" ");
                }
            }
        }
    }
    if print_char{
        print!("|{}|", ascii_string);
    }
}

fn main() -> std::io::Result<()> {
    let args:Vec<String> = args().collect();
    if args.len() < 2 {
        panic!("No arguments were given");
    }
    let mut print_ascii: bool = false;
    let mut path = String::new();
    for arg in args.iter() {
        if arg == "--help" {
            println!("{}", HELP_MESSAGE);
            return Ok(());
        }
        if arg == "--author" {
            println!("{}", AUTHORSHIP);
            return Ok(());
        }
        if arg == "--ascii" {
            print_ascii = true;
        }
        if Path::new(arg).exists() {
            path = arg.clone();
        }
    }

    if path == args[0] {
        panic!("No path was given");
    } 

    let read_result = read_raw_data(path);
    let raw_data = match  read_result {
        Ok(raw_data) => raw_data,
        Err(error) => panic!("Could not populate the buffer: {error}"),
    };

    let ascii_vector = create_char_vector(raw_data.clone()).unwrap();
    print_formated(raw_data, ascii_vector, print_ascii); 
    Ok(())
}
