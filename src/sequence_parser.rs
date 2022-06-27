use serde_json::Value;
use std::io::BufReader;
use std::fs;
use std::vec::Vec;
use std::error::Error;

pub fn load_sequence(file_name:&str) -> Result<Value,Box<dyn Error>> {
    let rdr = BufReader::new(fs::File::open(file_name).unwrap());
    let v = serde_json::from_reader(rdr)?;
    Ok(v)
}

pub fn get_url_patterns(v:&Value) -> Vec<String> {
    let mut patterns = Vec::new();
    let v = match v.get("patterns"){
        None => return patterns,
        Some(s) => s
    };
    if v.is_array(){
        for p in v.as_array().unwrap() {
            let v = match p.get("url_pattern") {
                None => return patterns,
                Some(s) => s
            };
            if v.is_string(){
                patterns.push(v.as_str().unwrap().to_string());
            }
        }
    }
    patterns
}

pub fn get_comment(v:&Value) -> String {
    let v = match v.get("comment"){
        None => return "".to_string(),
        Some(s) => s
    };
    if v.is_string(){
        return v.as_str().unwrap().to_string();
    }
    "".to_string()
}

pub fn get_version(v:&Value) -> i32 {
    let v =  match v.get("version") {
        None => return 0,
        Some(i) => i
    };
    if v.is_i64(){
        return v.as_i64().unwrap() as i32;
    }
    0
}
