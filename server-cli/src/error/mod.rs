use derive_more::From;
use iroh::endpoint;
use iroh_gossip::api::ApiError;
use tokio::task::JoinError;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    Bind(endpoint::BindError),

    #[from]
    Join(JoinError),

    #[from]
    Api(ApiError),

    #[from]
    Proto(proto::error::Error),
}
