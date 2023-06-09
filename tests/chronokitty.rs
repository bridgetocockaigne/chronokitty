use cucumber::World;

#[derive(Debug, Default, World)]
pub struct ChronoKittyWorld {}

#[tokio::main]
async fn main() {
    ChronoKittyWorld::run("tests/features/chronokitty.feature").await;
}
