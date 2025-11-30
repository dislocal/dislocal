use derive_more::From;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    Avro(apache_avro::Error),
}
