mod storage;
mod task;

#[tokio::main]
async fn main() {
    task::run().await;
    storage::run().await;
}
