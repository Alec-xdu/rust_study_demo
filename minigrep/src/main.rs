use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    let query_keyword = args[1].as_str();
    let filename = args[2].as_str();

}
