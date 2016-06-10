use std::io::Read;
use std::collections::BTreeMap;
use hyper::Url;
use hyper::method::Method;
use hyper::client::Request;
use hyper::error::Result as HttpResult;
use serde_json;
use serde_json::value::Value as JsonValue;


static GIPHY_SEARCH_URL: &'static str = "http://api.giphy.com/v1/gifs/search";

fn send_giphy_request(api_key: String, query: String) -> HttpResult<String> {
    let mut url = Url::parse(GIPHY_SEARCH_URL).unwrap();
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
pub fn request_gif(api_key: String, query: String) -> Option<String> {
    let json_body = send_giphy_request(api_key, query).expect("Couldn't get API info");
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
