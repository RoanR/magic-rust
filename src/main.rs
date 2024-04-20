use mtg_cards::{self, id_find};

#[tokio::main]
async fn main() {
    let card = id_find(386616).await;
    match card {
        Ok(c) => println!("\n{}", c.card),
        Err(e) => println!("All is not good?\n{:?}", e),
    }
}
