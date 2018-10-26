use hyper;
use hyper::{Request,Body};
use hyper::client::Client;
use futures::future;
use serde_json;
use serde_json::value::Value as JsonValue;
use hyper::rt::{Future,Stream};
use urlencoding;

static GIPHY_SEARCH_URL: &'static str = "http://api.giphy.com/v1/gifs/search";

fn send_giphy_request(api_key: &str, query: String) -> impl Future<Item=String, Error=()> {
    let query_url = format!("{}?q={}&api_key={}&limit=1",
                            GIPHY_SEARCH_URL,
                            urlencoding::encode(&query),
                            urlencoding::encode(api_key));
    let req = Request::builder()
        .uri(query_url)
        .method("GET")
        .body(Body::empty()).unwrap();

    let client = Client::new();
    client
        .request(req)
        .and_then(|res| {
            res.into_body()
                .fold(Vec::new(), |mut acc, chunk| {
                    acc.extend_from_slice(&*chunk);
                    future::ok::<Vec<u8>, hyper::Error>(acc)
                })
        })
        .map(|body_vec| {
            String::from_utf8(body_vec).unwrap()
        })
        .map_err(|err| { println!("Error with giphy response: {:?}", err) })
}

// [TODO] Better error handling (Result? don't use expect or unwrap)
pub fn request_gif(api_key: &str, query: String) -> impl Future<Item=String, Error=()> {
    println!("searching for giphy {}", query);
    send_giphy_request(api_key, query)
        .and_then(|json_body| {
            serde_json::from_str::<serde_json::Value>(&json_body)
                .ok()
                .and_then(|parsed| {
                    parsed.get("data")
                        .and_then(|data| {
                            match data {
                                &JsonValue::Array(ref vals) => vals.first(),
                                _ => None
                            }
                        })
                        .and_then(|first_data| {
                            first_data.get("images")
                        })
                        .and_then(|images| {
                            images.get("original")
                        })
                        .and_then(|gif_info| {
                            match gif_info.get("url").unwrap() {
                                &JsonValue::String(ref s) => Some(s),
                                _ => None
                            }
                        })
                        .map(|gif| {
                            gif.to_owned()
                        })
                })
                .ok_or_else(|| { println!("Couldn't parse JSON") })
        })
}
