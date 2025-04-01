use std::{env, process};

use minigrep::query;

fn main() {
    let v : Vec<String> = env::args().collect();
    let q = query::Query::new(&v).unwrap_or_else( |err|{
        println!("problem parsing arguments:{}", err);
        process::exit(1);
    });

    println!("Searching for {:#?}", q);

    if let Err(e) = minigrep::run(&q) {
        println!("Application error : {}", e);
        process::exit(1);
    }

}
