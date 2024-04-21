mod curdle;

#[tokio::main]
async fn main() {
    curdle::CurdleGame::new(5)
        .start_game()
        .await;
}
