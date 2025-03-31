use std::env;

use minigrep::query;

fn main() {
    let v : Vec<String> = env::args().collect();
    let q = query::Query::new(&v).unwrap();

    println!("Searching for {:#?}", q);

}
