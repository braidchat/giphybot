use conf;
use message;
use hyper::header::{CONTENT_TYPE,AUTHORIZATION};
use hyper::client::Client;
use hyper::{Response,Request,Body};
use hyper::error::Result as HttpResult;
use hyper::rt::Future;
use base64;

fn basic_auth(user: String, pass: String) -> String {
    format!("Basic {}", base64::encode(&format!("{}:{}", user, pass)))
}

pub fn send_braid_request(braid_conf: &conf::TomlConf, message: message::Message)
    -> HttpResult<Response<Body>>
{
    let api_url = braid_conf.get("api_url").unwrap().as_str().unwrap();
    let bot_id = braid_conf.get("app_id").unwrap().as_str().unwrap().to_owned();
    let token = braid_conf.get("token").unwrap().as_str().unwrap().to_owned();
    let body = message::encode_transit_msgpack(message);
    let req = Request::builder()
        .method("POST")
        .uri(api_url)
        .header(CONTENT_TYPE, "application/transit+msgpack")
        .header(AUTHORIZATION, basic_auth(bot_id, token))
        .body(Body::from(body))
        .unwrap();
    let client = Client::new();
    client.request(req).wait()
}
