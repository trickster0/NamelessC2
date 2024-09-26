use std::fs::File;
use std::io::Write;

pub fn upload(args: Vec<&str>) -> String {
    if let Ok(mut f) = File::create(args[0]){
        let file_data: Vec<u8> = base64::decode(args[1].as_bytes()).unwrap();
        f.write_all(file_data.as_slice());
        "File Operation Finished Successfully".to_string()
    }else {
        "Task failed successfully".to_string()
    }
   }