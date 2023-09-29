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
        if msg.content == "!hello" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
                error!("Error sending message: {:?}", e);
            }
        } else if msg.content == "!infos" {
            let mut embed = CreateEmbed::default();
            embed.title("Infos")
                    .description("Infos sur le bot")
                    .field("Créateur", "ncls-p", false)
                    .field("Langage", "Rust", false)
                    .field("Github", "https://github.com/ncls-p/IABD2.RS", false)
                    .field("Version", "0.1.0", false)
                    .field("Librairie", "Serenity", false)
                    .color(Colour::PURPLE)
                    .url("https://github.com/ncls-p/IABD2.RS")
                    .image("https://cdn.discordapp.com/attachments/1028352036049277060/1156246329736044676/download.png?ex=651445cf&is=6512f44f&hm=19e23e3d7ec03497772c97daccaa35cbb978df8e4c104414c0c2e63c547091f0&");

            if let Err(err) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(embed.clone()))
                .await
            {
                eprintln!("Error sending info message: {:?}", err);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
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
