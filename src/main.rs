use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::anyhow;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::utils::Colour;
use serenity::{async_trait, builder::CreateEmbed};
use shuttle_secrets::SecretStore;
use tracing::{error, info};

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        match msg.content.as_str() {
            "!hello" => hello_command(&ctx, &msg).await,
            "!infos" => infos_command(&ctx, &msg).await,
            "!help" => help_command(&ctx, &msg).await,
            "!github" => github_command(&ctx, &msg).await,
            "!rust" => rust_command(&ctx, &msg).await,
            "!selim" => selim_command(&ctx, &msg).await,
            "!ping" => ping_command(&ctx, &msg).await,
            _ => {}
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

async fn hello_command(ctx: &Context, msg: &Message) {
    if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
        error!("Error sending message: {:?}", e);
    }
}

async fn infos_command(ctx: &Context, msg: &Message) {
    let mut embed = CreateEmbed::default();
    embed.title("Infos")
        .description("Infos sur le bot")
        .field("Créateur", "ncls-p", false)
        .field("Langage", "Rust", false)
        .field(
            "Github",
            "https://github.com/ncls-p/rust-discord-bot-esgi",
            false,
        )
        .field("Version", "0.1.0", false)
        .field("Librairie", "Serenity", false)
        .color(Colour::PURPLE)
        .url("https://github.com/ncls-p/rust-discord-bot-esgi")
        .image(
            "https://cdn.discordapp.com/attachments/1028352036049277060/1156246329736044676/download.png?ex=651445cf&is=6512f44f&hm=19e23e3d7ec03497772c97daccaa35cbb978df8e4c104414c0c2e63c547091f0&",
        );

    if let Err(err) = msg
        .channel_id
        .send_message(&ctx.http, |m| m.set_embed(embed.clone()))
        .await
    {
        eprintln!("Error sending info message: {:?}", err);
    }
}

async fn help_command(ctx: &Context, msg: &Message) {
    let mut embed = CreateEmbed::default();
    embed
        .title("Commandes")
        .description("Liste des commandes")
        .field("*ping", "Renvoie pong", false)
        .field("*help", "Renvoie ce message", false)
        .field("*infos", "Renvoie des infos sur le bot", false)
        .field("*github", "Renvoie le lien du github", false)
        .field("*rust", "Renvoie le lien du rust", false);

    if let Err(err) = msg
        .channel_id
        .send_message(&ctx.http, |m| m.set_embed(embed.clone()))
        .await
    {
        eprintln!("Error sending help message: {:?}", err);
    }
}

async fn github_command(ctx: &Context, msg: &Message) {
    if let Err(err) = msg
        .reply(ctx, "https://github.com/ncls-p/rust-discord-bot-esgi")
        .await
    {
        eprintln!("Error sending github link: {:?}", err);
    }
}

async fn rust_command(ctx: &Context, msg: &Message) {
    if let Err(err) = msg.reply(ctx, "https://www.rust-lang.org/").await {
        eprintln!("Error sending rust link: {:?}", err);
    }
}

async fn selim_command(ctx: &Context, msg: &Message) {
    if let Err(err) = msg
        .reply(ctx, "gnn gnn gnn ça existe pas les lib rust pour discord")
        .await
    {
        eprintln!("Error sending selim message: {:?}", err);
    }
}

async fn ping_command(ctx: &Context, msg: &Message) {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let mut message = msg.reply(ctx.clone(), "Pinging...").await.unwrap();
    let latency_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        - time;

    let latency_ms = latency_timestamp.to_string();

    message
        .edit(&ctx, |m| {
            m.content(format!("Pong! 🏓 Latency: {}ms", latency_ms))
        })
        .await
        .unwrap();
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
