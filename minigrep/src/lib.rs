use std::error::Error;
use std::fs;
use crate::query::Query;

pub mod query {
    #[derive(Debug)]
pub struct Query<'a> {
    keyword: &'a str,
    filename: &'a str,
}

impl<'a> Query<'a> {
    pub fn new(args: &Vec<String>) -> Result<Query, &'static str> {
        let tup = match args.len() {
            3.. => (&args[1], &args[2]),
            _ => return Err("argument error!"),
        };
        Ok(Query {
            keyword: tup.0.as_str(),
            filename: tup.1.as_str(),
        })
    }

    pub fn from_string(keyword: &'a str, filename: &'a str) ->  Query<'a> {
        Query {
            keyword,
            filename,
        }
    }

    pub fn keyword(&self) -> &str {
        self.keyword
    }

    pub fn filename(&self) -> &str {
        self.filename
    }
}

pub fn parse_args(args: &Vec<String>) -> Query {
    Query {
        keyword: args[1].as_str(),
        filename: args[2].as_str(),
    }
}

}

pub fn run<'a>(query: &'a Query) -> Result<String, Box<dyn Error>>{
    let contents = fs::read_to_string(query.filename())?;
    println!("text:\n{}", contents);
    Ok(contents)
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Result<Vec<&'a str>, Box<dyn Error>> {
    let mut vs = Vec::new();
    for line in contents.lines() {
        if line.contains(query){
            vs.push(line);
        }
    }
    Ok(vs)
}

pub fn search_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str>{
    let mut result = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}
#[cfg(test)]
mod tests {
    use std::result;
    use super::*;

    #[test]
    fn one_result(){
        let query = Query::from_string("How", "poem.txt");
        let content = run(&query).unwrap();
        let result = search(query.keyword() , content.as_str()).unwrap();
        println!("{:#?}", result);
    }

    #[test]
    fn insensitive(){
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."],
        search_insensitive(query, contents)
        )
    }
}