use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;

#[command]
async fn kick(ctx: &Context, msg: &Message) -> CommandResult {
    let args = msg.content.split(" ").collect::<Vec<&str>>();
    let user = msg.mentions.first().unwrap();
    let guild = match msg.guild(&ctx.cache).await {
        Some(guild) => guild,
        None => {
            msg.reply(&ctx.http, "Unable to fetch guild!").await?;
            return Ok(());
        }
    };
    match args.binary_search(&"-r") {
        Ok(index) => {
            user.dm(&ctx.http, |m| {
                m.embed(|e| {
                    e.title(format!("You've Been Kicked from {}!", &guild.name))
                        .thumbnail(&user.avatar_url().unwrap())
                        .field("Reason: ", &args.as_slice()[index + 1..].join(" "), false)
                        .field(
                            "Kicker: ",
                            format!("{}#{}", &msg.author.name, &msg.author.discriminator),
                            false,
                        )
                })
            })
            .await?;
            guild
                .kick_with_reason(&ctx.http, user.id, args.as_slice()[index + 1..].join(" "))
                .await?;
        }
        Err(_) => {
            user.dm(&ctx.http, |m| {
                m.embed(|e| {
                    e.title(format!("You've Been Kicked from {}!", &guild.name))
                        .thumbnail(&user.avatar_url().unwrap())
                        .field("Reason: ", "None provided!", false)
                        .field(
                            "Kicker: ",
                            format!("{}#{}", &msg.author.name, &msg.author.discriminator),
                            false,
                        )
                })
            })
            .await?;
            guild.kick(&ctx.http, user.id).await?;
        }
    };
    Ok(())
}
