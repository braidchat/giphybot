use toml;

fn slurp(file_name: &str) -> Result<String, String> {
    use std::io::prelude::*;
    use std::fs::File;
    let mut s = String::new();
    match File::open(file_name).and_then(|mut f| { f.read_to_string(&mut s) }) {
        Ok(_) => Ok(s),
        Err(_) => Err("Couldn't open file to read".to_owned())
    }
}

pub fn load_credentials() {
    let toml_contents = slurp("conf.toml").expect("Couldn't load toml conf");
    let val =  toml::Parser::new(&toml_contents).parse().expect("Couldn't parse toml");
    println!("val = {:?}", val);
    let key = val.get("giphy")
        .and_then(|v| v.as_table())
        .and_then(|tbl| tbl.get("api_key"))
        .and_then(|key_v| key_v.as_str())
        .expect("Missing api_key value in giphy section");
    println!("key = {:?}", key);
}
