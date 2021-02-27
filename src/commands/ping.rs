use crate::ShardManagerContainer;
use serenity::client::bridge::gateway::ShardId;
use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    let shard_manager = match data.get::<ShardManagerContainer>() {
        Some(v) => v,
        None => {
            msg.reply(ctx, "There was a problem getting the shard manager")
                .await?;
            return Ok(());
        }
    };

    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;

    let runner = match runners.get(&ShardId(ctx.shard_id)) {
        Some(runner) => runner,
        None => {
            msg.reply(ctx, "No shard found").await?;
            return Ok(());
        }
    };

    let latency = match runner.latency {
        Some(latency) => latency.as_micros(),
        None => {
            msg.reply(ctx, "Latency hasn't been initialized yet!")
                .await?;
            return Ok(());
        }
    };
    msg.reply_ping(ctx, &format!("Pong at {}ms!", latency))
        .await?;
    Ok(())
}
