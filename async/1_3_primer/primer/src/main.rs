use futures::executor::block_on;
use async_std::task;
use std::time::Duration;

fn main() {
    block_on(async_main());
}

async fn async_main() {
    let f1 = learn_and_sing_song();
    let f2 = dance();

    futures::join!(f1, f2);
}

async fn learn_and_sing_song() {
    sing_song(learn_song().await).await;
}

async fn learn_song() -> String {
    println!("start learning");
    task::sleep(Duration::from_secs(1)).await;
    println!("end learning");
    "song".into()
}

async fn sing_song(song: String) {
    println!("start singing");
    task::sleep(Duration::from_secs(1)).await;
    println!("{}", song);
    println!("end singing");
}

async fn dance() {
    task::sleep(Duration::from_millis(400)).await;
    println!("dancing");
    task::sleep(Duration::from_millis(400)).await;
    println!("dancing");
    task::sleep(Duration::from_millis(400)).await;
    println!("dancing");
    task::sleep(Duration::from_millis(400)).await;
    println!("dancing");
    task::sleep(Duration::from_millis(400)).await;
    println!("dancing");
}