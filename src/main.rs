use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tonyfile = File::open("tonyfile.json")?;
    let reader = BufReader::new(tonyfile);

    let map: HashMap<String, Value> = serde_json::from_reader(reader)?;

    println!("{:#?}", map);

    Ok(())
}
