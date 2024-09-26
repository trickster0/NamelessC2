use std::env;

pub fn get_env() -> String {
    let mut result = String::new();
    for (key, value) in env::vars() {
        result.push_str(&format!("{}={}\n", key, value));
    }
    result
}