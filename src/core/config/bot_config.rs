use crate::core::{ext_lib::*, our_lib::*, std_lib::*};
use crate::commands::groups::*;
use crate::events::Handler;

// A modifier pour rajouter les éventuels autre groupes de commandes
pub fn get_framework(owners : HashSet<serenity::model::id::UserId>) -> StandardFramework {
    StandardFramework::new()
        .configure(|c| {
        c.prefix(&UTILS.bot_conf.prefix)
        .owners(owners)
    })
    .unrecognised_command(unknown_command)
    .help(&HELP)
    .group(&GENERAL_GROUP)
    //.group(&OTHER_GROUP)
}

//==========================================================================================================
pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub fn get_token() -> String {
    env::var(&UTILS.bot_conf.var_token).expect(&UTILS.bot_conf.expect_token)
}

pub async fn get_owners(token : &str) -> HashSet<serenity::model::id::UserId> {

    let http = Http::new_with_token(token);

    match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }

            owners
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    }
}

pub async fn get_client(token : &str, framework : StandardFramework) -> Client {
    let client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect(&UTILS.bot_conf.expect_client_builder);

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    client
}
