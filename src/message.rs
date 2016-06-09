use uuid::Uuid;
use std::io::Cursor;
use serde::Deserialize;
use rmp_serde::Deserializer;

type TransitUuid = (String, (i64, i64));

#[derive(Debug, PartialEq, Deserialize)]
struct Message {
    #[serde(rename="~:id")]
    id: TransitUuid,
    #[serde(rename="~:group-id")]
    group_id: TransitUuid,
    #[serde(rename="~:thread-id")]
    thread_id: TransitUuid,
    #[serde(rename="~:user-id")]
    user_id: TransitUuid,
    #[serde(rename="~:mentioned-user-ids")]
    mentioned_user_ids: Vec<TransitUuid>,
    #[serde(rename="~:mentioned-tag-ids")]
    mentioned_tag_ids: Vec<TransitUuid>,
    #[serde(rename="~:content")]
    content: String,
}

pub fn decode_msgpack_body(msgpack_buf: Vec<u8>) {
    let cur = Cursor::new(&msgpack_buf[..]);
    let mut deserializer = Deserializer::new(cur);
    match Deserialize::deserialize(&mut deserializer) {
        Result::Ok(res) => {
            let res: Message = res;
            println!("result = {:?}", res);
        }
        Result::Err(err) =>
            println!("error parsing: {:?}", err)
    }
}

