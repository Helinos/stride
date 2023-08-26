mod commands;
mod database;
mod responses;

use std::{collections::HashSet, env};

use commands::{music::{play::play, force_skip::force_skip, reorder::reorder, queue::queue, remove::remove}, settings::settings};
use hook::hook;
use lavalink_rs::{
    model::events,
    prelude::{LavalinkClient, NodeBuilder},
};
use poise::serenity_prelude as serenity;

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
            commands: vec![settings(), play(), force_skip(), reorder(), queue(), remove()],
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
}

#[hook]
async fn ready_event(client: LavalinkClient, session_id: String, event: &events::Ready) {
    client.delete_all_player_contexts().await.unwrap();
    info!("Ready event: {:?} -> {:?}", session_id, event);
}
