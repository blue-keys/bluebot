mod core;
mod commands;
mod events;

use self::core::our_lib::*;

// 752312576347602984 (BOT ID)
#[tokio::main]
async fn main() {
    let token = get_token();

    let owners = get_owners(&token).await;

    let framework = get_framework(owners);

    let mut client = get_client(&token, framework).await;

    if let Err(why) = client.start().await {
        eprintln!("Erreur : {:?}", why);
    }
} 