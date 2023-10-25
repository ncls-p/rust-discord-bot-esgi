use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::anyhow;
use chrono::{TimeZone, Utc};
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
            "!planning" => planning_command(&ctx, &msg).await,
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
async fn format_for_discord(ctx: &Context, msg: &Message, response: &str) {
    // Parsez la réponse JSON
    let data: serde_json::Value = serde_json::from_str(response).unwrap();

    // Vérifiez si la clé "result" existe et est un tableau
    if let Some(reservations) = data["result"].as_array() {
        let mut grouped_by_day: std::collections::HashMap<String, Vec<&serde_json::Value>> =
            std::collections::HashMap::new();
        for reservation in reservations {
            if let Some(start_date) = reservation["start_date"].as_i64() {
                let date = Utc
                    .timestamp_millis(start_date)
                    .format("%Y-%m-%d")
                    .to_string();
                grouped_by_day
                    .entry(date)
                    .or_insert_with(Vec::new)
                    .push(reservation);
            }
        }

        // Créez un embed pour chaque jour
        for (day, day_reservations) in grouped_by_day {
            let mut embed = CreateEmbed::default();
            embed
                .title(format!("Cours pour le {}", day))
                .color(Colour::from_rgb(114, 137, 218)) // Une couleur bleue Discord
                .footer(|f| f.text("Mis à jour le"))
                .timestamp(&*Utc::now().to_rfc3339());

            for courses_day in day_reservations {
                let course_name = courses_day["name"].as_str().unwrap_or("Inconnu");
                let teacher = courses_day["discipline"]["teacher"]
                    .as_str()
                    .unwrap_or("Inconnu");
                let start_time = Utc
                    .timestamp_millis(courses_day["start_date"].as_i64().unwrap_or(0))
                    .format("%H:%M")
                    .to_string();
                let end_time = Utc
                    .timestamp_millis(courses_day["end_date"].as_i64().unwrap_or(0))
                    .format("%H:%M")
                    .to_string();
                let room_name = courses_day["rooms"][0]["name"]
                    .as_str()
                    .unwrap_or("Inconnu");

                embed.field(
                    course_name,
                    format!(
                        "Enseignant: **{}**\nHeure: **{} - {}**\nSalle: **{}**",
                        teacher, start_time, end_time, room_name
                    ),
                    false,
                );
            }

            // Envoyez l'embed à Discord
            if let Err(err) = msg
                .channel_id
                .send_message(&ctx.http, |m| m.set_embed(embed))
                .await
            {
                eprintln!("Error sending embed message: {:?}", err);
            }
        }
    }
}

async fn planning_command(ctx: &Context, msg: &Message) {
    let username = "npierrot";
    let password = "Fk59vWay#-Fhviy55";

    // Encodage en base64 des identifiants pour l'authentification
    let auth_credentials = base64::encode(format!("{}:{}", username, password));

    // Tentative d'authentification et récupération du jeton d'accès
    let client = reqwest::Client::new();
    let response = client
        .get("https://authentication.kordis.fr/oauth/authorize?response_type=token&client_id=skolae-app")
        .header("Authorization", format!("Basic {}", auth_credentials))
        .send()
        .await;

    if let Ok(resp) = response {
        if let Some(redirect_url) = resp.headers().get("location") {
            let access_token: Vec<&str> = redirect_url
                .to_str()
                .unwrap()
                .split("access_token=")
                .collect();
            if access_token.len() > 1 {
                let token = access_token[1].split("&").next().unwrap();

                // Récupération du planning
                let start_timestamp = Utc.ymd(2023, 10, 25).and_hms(0, 0, 0).timestamp_millis();
                let end_timestamp = Utc.ymd(2023, 11, 25).and_hms(0, 0, 0).timestamp_millis();
                let agenda_url = format!(
                    "https://api.kordis.fr/me/agenda?start={}&end={}",
                    start_timestamp, end_timestamp
                );
                let agenda_response = client
                    .get(&agenda_url)
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await;

                if let Ok(agenda_resp) = agenda_response {
                    let agenda: String = agenda_resp.text().await.unwrap_or_default();
                    println!("{}", agenda);
                    format_for_discord(ctx, msg, &agenda).await;
                } else {
                    msg.channel_id
                        .say(&ctx.http, "Erreur lors de la récupération de l'agenda.")
                        .await
                        .unwrap();
                }
            } else {
                msg.channel_id
                    .say(&ctx.http, "Erreur lors de l'authentification.")
                    .await
                    .unwrap();
            }
        }
    } else {
        msg.channel_id
            .say(&ctx.http, "Erreur lors de la connexion à l'API Kordis.")
            .await
            .unwrap();
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
