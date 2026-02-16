use std::fs;
use std::env;

fn main() {
    println!("Hello, world!");


    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let src = &args[2];
    let dest = &args[3];

    println!("Processing graph in {file_path}. From {src} to {dest}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read this file?");

    println!("Content of graph\n{contents}");

    for line in contents.lines() {
        if line.starts_with("---") {
            break;
        }
        println!("Line {line}");
    }

}
