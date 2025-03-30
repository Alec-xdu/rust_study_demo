use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>>{
    let text = String::from("is are it an is list and again it it as");
    let mut map = HashMap::new();
    for tex in text.split_whitespace() {
        let count = map.entry(tex).or_insert(0);
        *count += 1;
    }
    let a = vec![1, 2, 3];
    let c = &a[1..2];
    println!("{:#?}", map);
    let file = File::open("some.txt").expect("2");
    println!("{:#?}", file);
    let mut s = String::new();
    // File::open("a.txt")?.read_to_string(&mut s)?;
    for i in 1..20 {
        print!("{}",i);
    }
    Ok(())
}

#[test]
fn read_file_content() ->Result<String, io::Error> {
    let mut s = String::new();
    File::open("a.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
