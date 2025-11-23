use proc_macro::TokenStream;

/// use std::sync::OnceLock;
/// use apache_avro::{Codec, Schema, Writer, ZstandardSettings};
///
/// pub fn write(hello: Hello) -> () {
///    let mut writer = Writer::with_codec(
///        schema(),
///        Vec::new(),
///        Codec::Zstandard(ZstandardSettings::default()),
///    );
///
///    writer.append_ser(hello).unwrap();
/// }
///
/// pub fn schema() -> &'static Schema {
///    static SCHEMA: OnceLock<Schema> = OnceLock::new();
///
///    SCHEMA.get_or_init(|| match Schema::parse_str(&HELLO_SCHEMA) {
///        Ok(schema) => schema,
///        Err(_) => todo!(),
///    })
/// }
#[proc_macro_derive(Avro, attributes(avro_schema_constant))]
pub fn derive_avro_write(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}
