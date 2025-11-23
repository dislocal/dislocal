use futures_lite::StreamExt;
use iroh::{Endpoint, SecretKey, protocol::Router};
use iroh_gossip::{
    Gossip, TopicId,
    api::{Event, GossipReceiver},
};
use proto::reader;
use server_cli::{Result, instrument};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    instrument::init();

    let secret_key = SecretKey::generate(&mut rand::rng());
    info!("server: secret key [{:?}]", secret_key.public());

    let endpoint = Endpoint::builder().secret_key(secret_key).bind().await?;
    info!("server: endpoint binded [{:?}]", endpoint.id());

    let gossip = Gossip::builder().spawn(endpoint.clone());

    let router = Router::builder(endpoint.clone())
        .accept(iroh_gossip::ALPN, gossip.clone())
        .spawn();

    let topic_id = TopicId::from_bytes(rand::random());
    let peer_ids = vec![];

    let (_sender, receiver) = gossip.subscribe(topic_id, peer_ids).await?.split();

    let hello = proto::Hello::new(
        proto::Role::Server,
        String::from(format!("{:?}", endpoint.id())),
    );

    let avro_hello = hello.write()?;

    reader(avro_hello);

    tokio::spawn(subscribe_loop(receiver));

    router.shutdown().await?;
    info!("server: router shutdown");

    endpoint.close().await;
    info!("server: endpoint closed");
    Ok(())
}

async fn subscribe_loop(mut receiver: GossipReceiver) -> Result<()> {
    while let Some(gossip_event) = receiver.try_next().await? {
        match gossip_event {
            Event::Received(message) => info!("Event Received: {:?}", &message),
            _ => info!("Gossip Event: {:?}", &gossip_event),
        }
    }
    Ok(())
}
