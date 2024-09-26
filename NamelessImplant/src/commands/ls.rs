use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;
use std::fmt::Write;
use chrono::{DateTime, Local, TimeZone};

fn epoch_to_datetime(epoch: i64) -> DateTime<Local> {
    Local.timestamp(epoch, 0)
}

fn list_elements(path: &str) -> Vec<(PathBuf, u64, SystemTime)> {
    let mut elements = vec![];
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    let modified_time = metadata.modified().unwrap_or_else(|_| SystemTime::UNIX_EPOCH);
                    let path = entry.path();
                    if metadata.is_dir() {
                        elements.insert(0, (path, 0, modified_time));
                    } else {
                        elements.push((path, metadata.len(), modified_time));
                    }
                }
            }
        }
    }
    elements
}

pub fn ls(dir: Vec<&str>) -> String {

    let elements = list_elements(&dir.join(" "));
    let mut result = String::new();
    for (path, size, modified_time) in elements {
        if let Some(name) = path.to_str() {
            let seconds_since_epoch = epoch_to_datetime(modified_time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64).format("%d-%m-%Y %H:%M:%S").to_string();
            if size == 0{
                writeln!(&mut result, "{} {} {}", seconds_since_epoch, "<DIR>", name).unwrap();
            }else {
                writeln!(&mut result, "{} {} {}", seconds_since_epoch, size, name).unwrap();
            } 
        }
    }
    if result.len() == 0{
        "[x] Path does not exist".to_string()
    }else {
        result
    }
}