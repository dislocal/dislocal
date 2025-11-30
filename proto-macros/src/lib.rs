use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

static MSG: &str = "Invalid #[avro_schema] format. Expected #[avro_schema(SCHEMA_CONST)]";

#[proc_macro_derive(Avro, attributes(avro_schema))]
pub fn derive_avro_write(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let name = input.ident;

    let meta = input
        .attrs
        .into_iter()
        .find(|x| x.path().is_ident("avro_schema"))
        .expect(MSG)
        .meta;

    let schema_ident = match meta {
        syn::Meta::List(meta_list) => meta_list.tokens.into_iter().next().expect(MSG),
        _ => panic!("{}", MSG),
    };

    let output = quote! {
        impl #name {
            pub fn schema() -> &'static ::apache_avro::Schema {
                static SCHEMA: ::std::sync::OnceLock<::apache_avro::Schema> = ::std::sync::OnceLock::new();

                SCHEMA.get_or_init(|| ::apache_avro::Schema::parse_str(#schema_ident).unwrap())
            }

            pub fn write(self) -> Vec<u8> {
                let mut writer = ::apache_avro::Writer::with_codec(
                    Self::schema(),
                    ::std::vec::Vec::new(),
                    ::apache_avro::Codec::Zstandard(::apache_avro::ZstandardSettings::default()),
                );

                writer.append_ser(self).unwrap();
                writer.into_inner().unwrap()
            }
        }
    };

    output.into()
}
