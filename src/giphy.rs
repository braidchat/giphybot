use std::collections::BTreeMap;
use hyper;
use hyper::{Request,Body};
use hyper::client::Client;
use serde_json;
use serde_json::value::Value as JsonValue;
use hyper::rt::{Future,Stream};
use urlencoding;

static GIPHY_SEARCH_URL: &'static str = "http://api.giphy.com/v1/gifs/search";

fn send_giphy_request(api_key: &str, query: String) -> Option<String> {
    let query_url = format!("{}?q={}&api_key={}&limit=1",
                            GIPHY_SEARCH_URL,
                            urlencoding::encode(&query),
                            urlencoding::encode(api_key));
    let req = Request::builder()
        .uri(query_url)
        .method("GET")
        .body(Body::empty()).unwrap();

    let client = Client::new();
    let resp: hyper::Response<hyper::Body> = client.request(req).wait().ok()?;

    let body = resp
        .into_body()
        .wait();
    let body_vec = body.fold(Vec::new(), |mut acc, res_chunk| {
            let chunk = res_chunk.unwrap();
            println!("Folding result chunk {:?}", chunk);
            acc.extend_from_slice(&*chunk);
            acc
        });
    String::from_utf8(body_vec).ok()
}

fn as_map(json: &JsonValue) -> Option<&BTreeMap<String, JsonValue>> {
    match json {
        &JsonValue::Object(ref obj) => Some(obj),
        _ => None
    }
}


// [TODO] Better error handling (Result? don't use expect or unwrap)
pub fn request_gif(api_key: &str, query: String) -> Option<String> {
    println!("searching for giphy {}", query);
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
