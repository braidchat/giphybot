use toml;
use std::io::Read;
use std::collections::BTreeMap;
use hyper::Url;
use hyper::method::Method;
use hyper::client::Request;
use hyper::error::Result as HttpResult;
use serde_json;
use serde_json::value::Value as JsonValue;

fn slurp(file_name: &str) -> Result<String, String> {
    use std::io::prelude::*;
    use std::fs::File;
    let mut s = String::new();
    match File::open(file_name).and_then(|mut f| { f.read_to_string(&mut s) }) {
        Ok(_) => Ok(s),
        Err(_) => Err("Couldn't open file to read".to_owned())
    }
}

fn load_credentials() -> String {
    let toml_contents = slurp("conf.toml").expect("Couldn't load toml conf");
    let val =  toml::Parser::new(&toml_contents).parse().expect("Couldn't parse toml");
    val.get("giphy")
        .and_then(|v| v.as_table())
        .and_then(|tbl| tbl.get("api_key"))
        .and_then(|key_v| key_v.as_str())
        .expect("Missing api_key value in giphy section")
        .to_owned()
}

static GIPHY_SEARCH_URL: &'static str = "http://api.giphy.com/v1/gifs/search";

fn send_giphy_request(query: String) -> HttpResult<String> {
    let mut url = Url::parse(GIPHY_SEARCH_URL).unwrap();
    let api_key = load_credentials();
    url.query_pairs_mut()
        .append_pair("q", &query[..])
        .append_pair("api_key", &api_key[..])
        .append_pair("limit", "1");
    let fresh_req = try!(Request::new(Method::Get, url));
    let streaming_req = try!(fresh_req.start());
    let mut resp = try!(streaming_req.send());
    let mut s = String::new();
    try!(resp.read_to_string(&mut s));
    Ok(s)
}

fn as_map(json: &JsonValue) -> Option<&BTreeMap<String, JsonValue>> {
    match json {
        &JsonValue::Object(ref obj) => Some(obj),
        _ => None
    }
}


// TODO: Better error handling (Result? don't use expect or unwrap)
pub fn request_gif(query: String) -> Option<String> {
    let json_body = send_giphy_request(query).expect("Couldn't get API info");
    match serde_json::from_str(&json_body[..]) {
        Ok(parsed) => {
            let parsed_json: BTreeMap<String, JsonValue> = parsed;
            parsed_json.get("data")
                .and_then(|data| {
                    match data {
                        &JsonValue::Array(ref vals) => vals.first(),
                        _ => None
                    }
                })
                .and_then(|first_data| {
                    as_map(first_data).and_then(|m| { m.get("images") })
                })
                .and_then(|images| {
                    as_map(images).and_then(|m| { m.get("original") })
                })
                .and_then(|gif_info| {
                    match as_map(gif_info).unwrap().get("url").unwrap() {
                        &JsonValue::String(ref s) => Some(s),
                        _ => None
                    }
                })
                .map(|gif| {
                    gif.to_owned()
                })
        }
        Err(e) => {
            println!("Failed to parse JSON: {:?}", e);
            None
        }
    }
}
