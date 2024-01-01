mod commands;
mod database;
mod responses;

use std::{collections::{HashSet, VecDeque}, env, time::Duration};

use commands::{music::{play::play, force_skip::force_skip, reorder::reorder, queue::queue, remove::remove, leave::leave, clear::clear}, settings::settings};
use hook::hook;
use lavalink_rs::{
    model::{events, track::TrackData},
    prelude::{LavalinkClient, NodeBuilder}, player_context::{self, TrackInQueue},
};
use poise::{
    Event,
    serenity_prelude::{self as serenity}
};

use database::DatabaseManager;
use songbird::SerenityInit;
use sqlx::ConnectOptions;
use tracing::info;

pub struct Data {
    database: DatabaseManager,
    lavalink: LavalinkClient,
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    // This will load the environment variables located at `./.env`, relative to
    // the CWD. See `./.env.example` for an example on how to structure this.
    dotenv::dotenv().expect("Failed to load .env file");

    tracing_subscriber::fmt::init();

    let token =
        env::var("DISCORD_TOKEN").expect("Expected a token in the environment (DISCORD_TOKEN)");

    let db_host = &env::var("MYSQL_HOST")
        .expect("Expected the database host in the environment (MYSQL_HOST)");
    let username = &env::var("MYSQL_USERNAME")
        .expect("Expected the database username in the environment (MYSQL_USERNAME)");
    let password = &env::var("MYSQL_PASSWORD")
        .expect("Expected the database password in the environment (MYSQL_PASSWORD)");
    let database =
        &env::var("MYSQL_DB").expect("Expected the database name in the environment (MYSQL_DB)");

    let lavalink_host = env::var("LAVALINK_HOST")
        .expect("Expected the lavalink host in the environment (LAVALINK_HOST)");
    let lavalink_password = env::var("LAVALINK_PASSWORD")
        .expect("Expected the lavalink host in the environment (LAVALINK_PASSWORD)");
    let lavalink_ssl = env::var("LAVALINK_SSL")
        .expect("Expected the lavalink ssl in the environment (LAVALINK_SSL)");

    // Connect to mysql database
    let connection_options = sqlx::mysql::MySqlConnectOptions::new();

    let database = DatabaseManager {
        pool: sqlx::mysql::MySqlPoolOptions::new()
            .max_connections(5)
            .connect_with(
                connection_options
                    .disable_statement_logging()
                    .host(db_host)
                    .username(username)
                    .password(password)
                    .database(database),
            )
            .await
            .expect("Couldn't connect to database."),
    };

    let framework = poise::Framework::builder()
        .token(token)
        .client_settings(|c| c.register_songbird())
        .options(poise::FrameworkOptions {
            owners: HashSet::from([serenity::UserId(126179145297166336)]),
            commands: vec![settings(), play(), force_skip(), reorder(), queue(), remove(), leave(), clear()],
            // Run before every command
            pre_command: |context| {
                Box::pin(async move {
                    let channel_name = &context
                        .channel_id()
                        .name(&context)
                        .await
                        .unwrap_or_else(|| "<unknown>".to_owned());
                    let author = &context.author().name;

                    info!(
                        "{} in {} used slash command '{}'",
                        author,
                        channel_name,
                        &context.invoked_command_name()
                    );
                })
            },
            // Run after every command that returned Ok
            post_command: |context| {
                Box::pin(async move {
                    info!(
                        "Executed command {} successfully",
                        context.command().qualified_name
                    );
                })
            },
            event_handler: |context, event, framework, data| {
                Box::pin(event_handler(context, event, framework, data))
            },
            ..Default::default()
        })
        .intents(serenity::GatewayIntents::all())
        .setup(|context, ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(context, &framework.options().commands).await?;

                let events = events::Events {
                    raw: Some(raw_event),
                    ready: Some(ready_event),
                    ..Default::default()
                };

                let node_local = NodeBuilder {
                    hostname: lavalink_host,
                    is_ssl: lavalink_ssl.as_str() == "true",
                    events: events::Events::default(),
                    password: lavalink_password,
                    user_id: context.cache.current_user_id().into(),
                    session_id: None,
                };

                let client = LavalinkClient::new(events, vec![node_local]);

                client.start().await;

                info!("Bot logged in as {}", ready.user.name);

                Ok(Data {
                    database,
                    lavalink: client,
                })
            })
        });

    framework.run().await.unwrap();
}

#[hook]
async fn raw_event(_: LavalinkClient, session_id: String, event: &serde_json::Value) {
    if event["op"].as_str() == Some("event") || event["op"].as_str() == Some("playerUpdate") {
        info!("Raw event: {:?} -> {:?}", session_id, event);
    }

    if event["op"].as_str() == Some("event") {
        match event["type"].as_str() {
            Some("WebSocketClosedEvent") => {

            },
            Some(_) => (),
            None => (),
        }
    }
}

#[hook]
async fn ready_event(client: LavalinkClient, session_id: String, event: &events::Ready) {
    client.delete_all_player_contexts().await.unwrap();
    info!("Ready event: {:?} -> {:?}", session_id, event);
}

async fn event_handler(
    context: &serenity::Context,
    event: &Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        Event::VoiceServerUpdate { update } => {
            let guild_id = update.guild_id.unwrap();
            
            let manager = songbird::get(context)
                .await
                .unwrap()
                .clone();

            let call_option = manager.get(guild_id);

            let Some(call) = call_option else {
                return Ok(());
            };

            let connection_info = call.lock().await.current_connection().unwrap().clone();
            let lava_client = data.lavalink.clone();
            
            let Some(player_context) = lava_client.get_player_context(guild_id) else {
                return Ok(());
            };

            let track_position_queue: Option<(TrackData, u64, VecDeque<TrackInQueue>)>;
            
            let player = player_context.get_player().await?;
            if let Some(track) = player.track {
                let position = player.state.position;
                let queue = player_context.get_queue().await?;
                track_position_queue = Some((track, position, queue));
            } else {
                track_position_queue = None;
            }

            lava_client.delete_player(guild_id).await?;
            lava_client.create_player_context(guild_id, connection_info).await?;

            let Some(player_context) = lava_client.get_player_context(guild_id) else {
                return Ok(());
            };

            if let Some((track, position, queue)) = track_position_queue {
                player_context.play_now(&track).await?;
                player_context.set_position(Duration::from_millis(position)).await?;
                player_context.set_queue(player_context::QueueMessage::Replace(queue))?;
            }
        }

        Event::VoiceStateUpdate { old: _, new } => {
            let current_user_id = context.cache.current_user_id();

            if new.user_id != current_user_id {
                return Ok(());
            }

            if new.channel_id.is_some() {
                return Ok(());
            }

            let lava_client = data.lavalink.clone();

            let guild_id = new.guild_id.unwrap();
            lava_client.delete_player(guild_id).await?;
        }

        _ => ()
    }
    
    Ok(())
}