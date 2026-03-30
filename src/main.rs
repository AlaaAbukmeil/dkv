mod store;
mod node;
mod rpc;

use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::Server;

use rpc::kvstore::kv_store_server::KvStoreServer;
use rpc::KvService;
use store::Store;
use node::Node;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "50051".to_string());

    let addr = format!("127.0.0.1:{}", port).parse()?;

    let node = Node::new(
        format!("node-{}", port),
        vec![], // add peer addresses here later
    );

    let service = KvService {
        store: Arc::new(Store::new()),
        node: Arc::new(Mutex::new(node)),
    };

    println!("Node listening on {}", addr);

    Server::builder()
        .add_service(KvStoreServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}