use conf;
use message;
use hyper::header::{Headers,ContentType,Authorization,Basic};
use hyper::client::{Client,Response};
use hyper::error::Result as HttpResult;
use mime::{Mime,TopLevel,SubLevel};


pub fn send_braid_request(braid_conf: &conf::TomlConf, message: message::Message)
    -> HttpResult<Response>
{
    let api_url = braid_conf.get("api_url").unwrap().as_str().unwrap();
    let bot_id = braid_conf.get("app_id").unwrap().as_str().unwrap().to_owned();
    let token = braid_conf.get("token").unwrap().as_str().unwrap().to_owned();
    let body = message::encode_transit_msgpack(message);
    let client = Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType(Mime(TopLevel::Application,
                              SubLevel::Ext("transit+msgpack".to_owned()),
                              vec![])));
    headers.set(Authorization(Basic{username: bot_id, password: Some(token)}));
    client.put(api_url)
        .body(&body[..])
        .headers(headers)
        .send()
}
