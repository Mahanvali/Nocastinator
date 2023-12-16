use anyhow::Context as _;
use poise::serenity_prelude::{self as serenity, OnlineStatus};
use shuttle_poise::ShuttlePoise;
use shuttle_secrets::SecretStore;
use serenity::model::id::UserId;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Responds with "world!"
#[poise::command(slash_command)]
async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("world!").await?;
    Ok(())
}

#[shuttle_runtime::main]
async fn poise(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> ShuttlePoise<Data, Error> {
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![hello()],
            event_handler: |ctx, event, _framework, data| {
                Box::pin(async move { event_handler(ctx, event, _framework, data).await})
            },
            ..Default::default()
        })
        .token(discord_token)
        .intents(serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::GUILDS | serenity::GatewayIntents::GUILD_PRESENCES | serenity::GatewayIntents::GUILD_MEMBERS)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build()
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(framework.into())
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data
) -> Result<(), Error> {
    match event {
        poise::Event::Ready { data_about_bot } => {
            println!("Woop woop! {} is online", data_about_bot.user.name)
        }


        poise::Event::PresenceUpdate { new_data } => {
             let myuserid = "USER ID HERE"
             if new_data.user.id == myuserid {
                 if new_data.status == OnlineStatus::Online {
                    tokio::time::sleep(tokio::time::Duration::from_secs(2 * 60)).await;
                    let user = UserId(764834445722386432);
                    let dm_channel = user.create_dm_channel(&ctx.http).await?;
                    let message = "Buddy boy get back to work.";
                    dm_channel.say(&ctx.http, message).await?;
                 }
             }
        }
        _ => {}
    }
    Ok(())
}
