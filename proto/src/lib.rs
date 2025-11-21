use std::sync::OnceLock;

use apache_avro::{Codec, Schema, Writer, ZstandardSettings};
use serde::Serialize;

static HELLO_SCHEMA: &str = r#"
        {
            "name": "hello",
            "type": "record",
            "fields": [
                {"name": "role", "type": "enum", "symbols": ["Server", "Client"]},
                {"name": "node_id", "type": "string"}
            ]
        }
    "#;

pub fn write(hello: Hello) -> () {
    let mut writer = Writer::with_codec(
        schema(),
        Vec::new(),
        Codec::Zstandard(ZstandardSettings::default()),
    );

    writer.append_ser(hello).unwrap();
}

#[derive(Debug, Serialize)]
pub struct Hello {
    role: Role,
    node_id: String,
}

pub fn schema() -> &'static Schema {
    static SCHEMA: OnceLock<Schema> = OnceLock::new();

    SCHEMA.get_or_init(|| match Schema::parse_str(&HELLO_SCHEMA) {
        Ok(schema) => schema,
        Err(_) => todo!(),
    })
}

#[derive(Debug, Serialize)]
pub enum Role {
    Server,
    Client,
}
