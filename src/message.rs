use std::io::Cursor;
use byteorder::{WriteBytesExt,BigEndian};
use uuid::Uuid;
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

fn transit_to_uuid(transit_uuid: &TransitUuid) -> Uuid {
    assert!(transit_uuid.0 == "~#u", "Mis-tagged transit");
    let mut wrtr = vec![];
    let hi64 = (transit_uuid.1).0;
    let lo64 = (transit_uuid.1).1;
    wrtr.write_i64::<BigEndian>(hi64).unwrap();
    wrtr.write_i64::<BigEndian>(lo64).unwrap();
    let mut bytes: [u8; 16] = [0; 16];
    for i in 0..wrtr.len() {
        bytes[i] = wrtr[i];
    }
    println!("bytes = {:?}", bytes);
    Uuid::from_bytes(&bytes).ok().unwrap()
}

pub fn decode_msgpack_body(msgpack_buf: Vec<u8>) {
    let cur = Cursor::new(&msgpack_buf[..]);
    let mut deserializer = Deserializer::new(cur);
    match Deserialize::deserialize(&mut deserializer) {
        Result::Ok(res) => {
            let res: Message = res;
            println!("result = {:?}", res);
            let id_uuid = transit_to_uuid(&res.id);
            println!("id = {:?} = {:?}", res.id, id_uuid);
        }
        Result::Err(err) =>
            println!("error parsing: {:?}", err)
    }
}

