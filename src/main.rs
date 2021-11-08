//! Requires the 'framework' feature flag be enabled in your project's
//! `Cargo.toml`.
//!
//! This can be enabled by specifying the feature in the dependency section:
//!
//! ```toml
//! [dependencies.serenity]
//! git = "https://github.com/serenity-rs/serenity.git"
//! features = ["framework", "standard_framework"]
//! ```
mod commands;
mod util;

use commands::{
    meta::*, 
    settings::*, 
    music::{
        clear::*,
        force_skip_to::*,
        force_skip::*,
        join::*,
        leave::*,
        now_playing::*,
        play::*,
        queue::*,
        remove::*,
        reorder::*,
        shuffle::*,
        skip::*,       
    }
};

use util::database::DatabaseTool;

use std::{collections::{HashSet, HashMap}, env, sync::Arc};
use sqlx::{sqlite::SqlitePool};
use uuid::Uuid;

use songbird::SerenityInit;

use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::{
        standard::{
            macros::{
                group,
                hook,
            },
        },
        StandardFramework
    },
    http::Http,
    model::{
        event::ResumedEvent,
        gateway::Ready,
        channel::Message,
        id::{
            UserId,
            GuildId,
        }
    },
    prelude::*,
};

use tokio::sync::RwLock;
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, FmtSubscriber};



#[group]
#[commands(
    ping, help, settings, join, leave, play, force_skip, clear, now_playing, skip, todo, play_top, play_skip, queue, remove, reorder, force_skip_to, shuffle, source
)]
struct General;


struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}


pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Database;

impl TypeMapKey for Database {
    type Value = DatabaseTool;
}

struct GuildSkipVotes;

impl TypeMapKey for GuildSkipVotes {
    type Value = Arc<RwLock<HashMap<GuildId, HashSet<UserId>>>>;
}

struct UserTrackMap;

impl TypeMapKey for UserTrackMap {
    type Value = Arc<RwLock<HashMap<GuildId, HashMap<Uuid, UserId>>>>;
}


#[hook]
async fn dynamic_prefix(ctx: &Context, msg: &Message) -> Option<String> {
    let guild_id = match msg.guild_id {
        Some(id) => id.0,
        None => 0,
    };

    let prefix: String;

    if guild_id != 0 {
        let data = ctx.data.read().await;
        let database = data.get::<Database>().expect("Expected Database in TypeMap");

        database.lookup("guild_settings", guild_id).await;
        prefix = database.retrieve_str("guild_settings", "prefix", guild_id).await;
    } else {
        prefix = "!".to_string();
    }

    Some(prefix)
}



#[tokio::main]
async fn main() {
    // This will load the environment variables located at `./.env`, relative to
    // the CWD. See `./.env.example` for an example on how to structure this.
    dotenv::dotenv().expect("Failed to load .env file");

    // Initialize the logger to use environment variables.
    //
    // In this case, a good default is setting the environment variable
    // `RUST_LOG` to debug`.
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to start the logger");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);

    // We will fetch your bot's owners and id
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };
    
    // Create the framework
    let framework = StandardFramework::new()
        .configure(|c| {
            c
            .owners(owners)
            .on_mention(Some(bot_id))
            .dynamic_prefix(dynamic_prefix)
        })
        .group(&GENERAL_GROUP);

    let database = DatabaseTool {pool: SqlitePool::connect("sqlite:main.db").await.expect("Database may not exist")};
    let guild_skip_votes = Arc::new(RwLock::new(HashMap::new()));
    let user_track_map = Arc::new(RwLock::new(HashMap::new()));

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .register_songbird()
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<Database>(database);
        data.insert::<GuildSkipVotes>(guild_skip_votes);
        data.insert::<UserTrackMap>(user_track_map);
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}

