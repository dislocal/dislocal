use proto_macros::Avro;
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

#[derive(Debug, Serialize, Avro)]
#[avro_schema_constant(HELLO_SCHEMA)]
pub struct Hello {
    role: Role,
    node_id: String,
}

#[derive(Debug, Serialize)]
pub enum Role {
    Server,
    Client,
}
