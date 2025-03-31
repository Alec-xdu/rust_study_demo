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