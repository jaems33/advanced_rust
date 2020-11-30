use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("Key was none there");
    let value = arguments.next().expect("No values was found");

    let mut db = Database::new().expect("Creating database failed.");
    db.insert(key, value);
    db.write();
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        
        // Read the kv.db file
        let contents = match std::fs::read_to_string("kv.db") {
            Result::Ok(contents_string) => contents_string,
            Err(error) => {
                return Result::Err(error);
            }
        };

        // Short form of doing a similar thing as above
        let contents = std::fs::read_to_string("kv.db")?;
        let mut map = HashMap::new();
        // parse the string
        // Hands references to points of contents
        for line in contents.lines() {
            // .split_once() is an unstable method
            let mut chunks_iter = line.splitn(2, '\t');
            let key = chunks_iter.next().expect("No key available.");
            let value = chunks_iter.next().expect("No value available.");
            map.insert(key.to_owned(), value.to_owned());
        }
        // populate the map
        Ok(Database{
            map: map
        })
    }

    fn insert(&mut self, mut key: String, value: String){
        &self.map.insert(key, value);
    }

    fn write(&self){
        let mut output: String = "".to_owned();
        for (key, value) in &self.map {
            let contents = format!("{}\t{}\n", key, value);
            output.push_str(&contents.to_owned());
        }
        std::fs::write("kv.db", output).unwrap();
    }

    
}