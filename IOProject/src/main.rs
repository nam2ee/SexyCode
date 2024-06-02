use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");
    // 변수가 많아질 수록 추적이 힘들다 

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: &String,
    filename: &String
}

fn parse_config(list: &[String]) -> Config
{
    let query = &list[0];
    let filename = &list[1];

    let result = Config{
        query,filename,
    };
    result
}