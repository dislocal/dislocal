use derive_more::From;
use iroh::endpoint;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    Bind(endpoint::BindError),
}
