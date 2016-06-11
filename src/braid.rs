use message;
use conf;
use hyper::Url;
use hyper::header::{Headers,ContentType};
use hyper::client::{Client,Response};
use hyper::error::Result as HttpResult;
use mime::{Mime,TopLevel,SubLevel};


pub fn send_braid_request(braid_conf: &conf::TomlConf, message: message::Message) -> HttpResult<Response> {
    let api_url = braid_conf.get("api_url").unwrap().as_str().unwrap();
    let body = message::encode_transit_msgpack(message);
    let client = Client::new();
    let mut headers = Headers::new();
    headers.set(
        ContentType(Mime(TopLevel::Application,
                         SubLevel::Ext("transit+msgpack".to_owned()),
                         vec![]))
        );
    client.put(api_url)
        .body("foo")
        .send()

}
