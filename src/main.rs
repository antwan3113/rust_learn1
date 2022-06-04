use std::collections::HashMap;
use std::io;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    println!("The key is {} and the value is {}", key, value);

    let contents = format!("{}\t{}\n", key, value);
    std::fs::write("kv.db", contents).unwrap();

    let mut database = DataBase::new().expect("Database::new() crashed");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);
    // database.flush().unwrap();

}

struct DataBase {
    map: HashMap<String, String>,
    flush: bool,
}


impl DataBase {
    fn new() -> Result<DataBase, std::io::Error> {
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;
        // parse the string
        for line in contents.lines(){
            let mut chunks = line.splitn(2,'\t');
            let key= chunks.next().expect("No key");
            let value = chunks.next().expect("No value");
            map.insert(key.to_owned(), value.to_owned());

        }
        // populate the map
        Ok(DataBase { map, flush: false })
    }

    fn insert(&mut self, key: String, value: String){
        self.map.insert(key, value);
    }

    fn flush(mut self) -> std::io::Result<()> {
        self.flush = true;
        do_flush(&self)
    }
}

impl Drop for DataBase {
    fn drop(&mut self) {
        if !self.flush {
            let _ = do_flush(self);
        }
    }
}

fn do_flush(database: &DataBase) -> std::io::Result<()> {
    let mut contents = String::new();
    for (key, value) in &database.map {
        contents.push_str(key);
        contents.push('\t');
        contents.push_str(value);
        contents.push('\n');
    }
    std::fs::write("kv.db", contents)
}
