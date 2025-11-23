use std::sync::OnceLock;

use apache_avro::{Codec, Reader, Schema, Writer, ZstandardSettings, from_value};
use serde::{Deserialize, Serialize};

use crate::result::Result;

pub mod error;
pub mod result;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Hello {
    role: Role,
    node_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    Server,
    Client,
}

impl Hello {
    pub fn new(role: Role, node_id: String) -> Self {
        Self { role, node_id }
    }

    pub fn write(self) -> Result<Vec<u8>> {
        let mut writer = Writer::with_codec(
            Self::schema(HELLO_SCHEMA),
            Vec::new(),
            Codec::Zstandard(ZstandardSettings::default()),
        );

        writer.append_ser(self)?;
        Ok(writer.into_inner()?)
    }
    pub fn schema(schema: &'static str) -> &'static Schema {
        static SCHEMA: OnceLock<Schema> = OnceLock::new();

        SCHEMA.get_or_init(|| match Schema::parse_str(schema) {
            Ok(schema) => schema,
            Err(_) => todo!(),
        })
    }
}

pub fn reader(input: Vec<u8>) {
    let reader = Reader::new(&input[..]).unwrap();
    let schema = reader.writer_schema();

    match schema.name() {
        Some(_) => {
            for value in reader {
                println!("{:?}", from_value::<Hello>(&value.unwrap()));
            }
        }
        None => todo!(),
    }
}
