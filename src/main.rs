use mtg_cards;

#[tokio::main]
async fn main() {
    let card = mtg_cards::id_find(386616).await;
    match card {
        Ok(c) => println!("All is good!\n{:?}", c),
        Err(e) => println!("All is not good?\n{:?}", e),
    }
}
