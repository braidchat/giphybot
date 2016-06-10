use toml;
use std::collections::BTreeMap;
use std::io::Read;

fn slurp(file_name: &str) -> Result<String, String> {
    use std::io::prelude::*;
    use std::fs::File;
    let mut s = String::new();
    match File::open(file_name).and_then(|mut f| { f.read_to_string(&mut s) }) {
        Ok(_) => Ok(s),
        Err(_) => Err("Couldn't open file to read".to_owned())
    }
}

pub fn load_conf(file_name: &str) -> Result<BTreeMap<String, toml::Value>, String> {
    let contents = try!(slurp(file_name).map_err(|e| e.to_string()));
    try!(toml::Parser::new(&contents).parse().ok_or("Couldn't parse TOML"))
}

pub fn get_conf_val(conf: BTreeMap<String, toml::Value>, group: &str, key: &str) -> Option<String> {
    conf.get(group)
        .and_then(|v| v.as_table())
        .and_then(|tbl| tbl.get(key))
        .and_then(|key_v| key_v.as_str())
        .map(|s| s.to_owned())
}
