use std::fs::File;
use std::path::Path;
use std::io::{BufReader, Read};

pub fn cat(path: Vec<&str>) -> String {
    if Path::new(path[0]).exists() {
        if let Ok(file) = File::open(path[0]){
            let mut reader = BufReader::new(file);
            let mut contents = Vec::new();
            reader.read_to_end(&mut contents);
    
            let encoded = base64::encode(&contents);
            encoded
        }else {
            base64::encode("[x] File does not exist")
        }

    } else {
        base64::encode("[x] File does not exist")
    }
}