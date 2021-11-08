use crate::{
    util::embeds,
    Database, 
    GuildSkipVotes
};

use std::{
    process::Stdio,
    sync::Arc,
};
use tokio::{process::Command as IOCommand};

use serenity::{
    client::Context,
    model::{channel::Message, guild::Guild, guild::Role, id::RoleId, prelude::GuildId},
    prelude::Mutex,
};

use songbird::{
    input::error::Result,
    Call, 
    Songbird
};



pub struct MusicCommand<'a> {
    pub ctx: &'a Context,
    pub msg: &'a Message,
    pub guild: Guild,
    pub guild_id: GuildId,
    pub dj_role: Option<Role>,
}

impl<'a> MusicCommand<'a> {
    pub async fn new(ctx: &'a Context, msg: &'a Message) -> MusicCommand<'a> {
        // Get the guild
        let guild = msg.guild(&ctx.cache).await.unwrap();

        // Get the dj role
        let data = ctx.data.read().await;
        let database = data.get::<Database>().expect("Expected Database in TypeMap");
        let dj_role_i64: i64 = database.retrieve_int("guild_settings", "dj_role_id", guild.id.0).await;
        let dj_role: Option<Role>;
        if dj_role_i64 == 0 {
            dj_role = None; // Role Doesn't exist
        } else {
            let dj_role_id = RoleId(dj_role_i64 as u64);
            dj_role = dj_role_id.to_role_cached(&ctx.cache).await;
        }

        Self { ctx: ctx, msg: msg, guild_id: guild.id, guild: guild, dj_role: dj_role }
    }

    pub async fn get_manager(&self) -> Option<Arc<Songbird>> {
        // Get the channel the user is connected to
        let channel_id = self.guild.voice_states.get(&self.msg.author.id).and_then(|voice_state| voice_state.channel_id);

        // Check if the user is in a voice channel
        match channel_id {
            Some(_) => {
                // User is in a voice channel
                // Get the songbird voice client
                let manager = songbird::get(self.ctx).await.expect("Songbird Voice client placed in at initialisation.").clone();
                // Check if the bot thinks its still in a voice channel
                if let Some(handler_lock) = manager.get(self.guild_id) {
                    // Will only return Some if the bot has joined a channel before
                    let handler = handler_lock.lock().await;
                    if None == handler.current_connection() {
                        // Clean up the bot if it isn't actually connected
                        let _ = handler.queue().stop();
                        self.clear_votes().await;
                        self.clear_utmap().await;
                    }
                }
                return Some(manager);
            }
            None => {
                // User is not in a voice channel
                embeds::user_missing(self.ctx, self.msg).await;
                return None;
            }
        };
    }

    pub async fn has_dj(&self) -> bool {
        if let Some(role) = &self.dj_role {
            let has_role = self.msg.author.has_role(&self.ctx.http, self.guild_id, role.id.0).await;
            match has_role {
                Ok(has_role) => {
                    if has_role {
                        // User has the role
                        return true;
                    } else {
                        // User doesn't have the role
                        embeds::need_dj(self.ctx, self.msg, role).await;
                        return false;
                    }
                }
                Err(_) => {
                    return false; // If there's an http or json error it's safe to assume the user is trying to fuck with the bot
                }
            }
        } else {
            embeds::no_dj(self.ctx, self.msg).await;
            return false; // Role doesn't exist
        }
    }

    // Check to see if the bot is currently connected to a voice channel
    // Returns the handler_lock if it is
    // Returns none if it isn't
    pub async fn connected(&self) -> Option<Arc<Mutex<Call>>> {
        if let Some(manager) = self.get_manager().await {
            if let Some(handler_lock) = manager.get(self.guild_id) {
                return Some(handler_lock);
            } else {
                embeds::bot_missing(self.ctx, self.msg).await;
                return None;
            }
        } else {
            return None;
        }
    }

    pub async fn clear_votes(&self) {
        let data = self.ctx.data.read().await;
        let guild_skip_votes_lock = data.get::<GuildSkipVotes>().expect("Expected GuildSkipVotes in TypeMap");
        let mut guild_skip_votes = guild_skip_votes_lock.write().await;
        guild_skip_votes.remove(&self.guild_id);
    }

    pub async fn clear_utmap(&self) {
        let data = self.ctx.data.read().await;
        let user_track_map_lock = data.get::<GuildSkipVotes>().expect("Expected GuildSkipVotes in TypeMap");
        let mut user_track_map = user_track_map_lock.write().await;
        user_track_map.remove(&self.guild_id);
    }
}


// There may be an issue where once ytdl encounters an unavailable song it'll stop aggregating ids from that point on.
pub async fn ytdl_playlist(uri: String) -> Result<Vec<String>> {
    let ytdl_args = [
        "-R", 
        "infinite", 
        "--ignore-config", 
        "--no-warnings",
        "--flat-playlist",
        "--get-id",
        &uri, 
        "-o", 
        "-"
    ];

    let youtube_dl_output = IOCommand::new("youtube-dl")
        .args(&ytdl_args)
        .stdin(Stdio::null())
        .output()
        .await?;

    let output_vec = youtube_dl_output.stderr;
    let output_str = std::str::from_utf8(&output_vec).unwrap_or_default();
    let mut ids: Vec<String> = output_str
        .lines()
        .map(|x| format!("https://www.youtube.com/watch?v={}", x))
        .collect();

    // let mut ids: Vec<&str> = split.collect();
    for (index, id) in ids.clone().iter().enumerate() {
        if id.contains("available") { // A phrase that's present in both kinds of strings an errored id returns
            ids.remove(index);
        } 
    }

    Ok(ids)
}
