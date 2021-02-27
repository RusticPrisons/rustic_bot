mod commands;
use commands::{ban::*, kick::*, ping::*};
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{macros::group, StandardFramework};

use serenity::client::bridge::gateway::ShardManager;

use serenity::prelude::*;

use serenity::model::guild::Member;
use serenity::model::id::GuildId;
use std::sync::Arc;

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

#[group]
#[commands(ping)]
struct General;

#[group]
#[commands(ban, kick)]
struct Moderation;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, _: GuildId, mut member: Member) {
        match member.add_role(&ctx.http, 811279410597986344).await {
            Ok(_) => (),
            Err(err) => println!("failed to add role to user reason: {}!", err),
        };
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("."))
        .group(&GENERAL_GROUP);

    let token = "ODExMjg4MDA0NDY3NDkwODI2.YCwA1Q.0XfjnnMg9Rs6z4vrZPRuZBgKvNM";
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");
    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }
    if let Err(why) = client.start_autosharded().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
