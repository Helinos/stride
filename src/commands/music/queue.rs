use crate::{
    util::{
        embeds,
        misc::{check_msg, seconds_to_string},
        music::MusicCommand,
    },
    UserTrackMap,
};

use std::collections::HashMap;

use uuid::Uuid;

use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{channel::Message, id::UserId},
    utils::Color,
};

use songbird::tracks::TrackHandle;

#[command]
#[only_in(guilds)]
#[aliases(q)]
async fn queue(ctx: &Context, msg: &Message) -> CommandResult {
    let mc = MusicCommand::new(ctx, msg).await;
    let handler_lock = mc.connected().await;

    if mc.connected().await.is_some() {
        let handler_lock = handler_lock.unwrap();
        let handler = handler_lock.lock().await;
        let queue = handler.queue();

        // Check if there's any music playing
        let current_track = match queue.current() {
            Some(current_track) => current_track,
            None => {
                embeds::not_playing(ctx, msg).await;
                return Ok(());
            },
        };

        let mut queue_embed = serenity::builder::CreateEmbed::default();
        queue_embed.color(Color::from_rgb(149, 165, 166));
        let mut queue_string = String::default();

        // Get the user track map from data
        let data = ctx.data.read().await;
        let user_track_map_lock = data
            .get::<UserTrackMap>()
            .expect("Expected UserTrackMap in TypeMap");
        let user_track_map = user_track_map_lock.read().await;
        let map = user_track_map.get(&mc.guild_id).unwrap();

        let (title, url, duration, mention) = get_track_info(&current_track, map).await;
        queue_string.push_str(
            format!(
                "**Currently Playing**\n[{}]({}) | `{}`\n[{}]\n\n",
                title,
                url,
                seconds_to_string(duration).await,
                mention
            )
            .as_str(),
        );
        let queue_length = queue.current_queue().len();
        if queue_length >= 2 {
            queue_string.push_str("**Up Next**\n");
            // We don't want to get more than 10 songs
            let mut queue_length_adj = queue_length;
            if queue_length > 10 {
                queue_length_adj = 10
            };

            let mut total_length: u64 = 0;
            for (count, track) in queue.current_queue()[1..queue_length_adj]
                .iter()
                .enumerate()
            {
                let (title, url, duration, mention) = get_track_info(&track, map).await;
                total_length = total_length + duration;
                queue_string.push_str(
                    format!(
                        "{}. [{}]({}) | `{}`\n[{}]\n\n",
                        count + 1,
                        title,
                        url,
                        seconds_to_string(duration).await,
                        mention
                    )
                    .as_str(),
                );

                // If the string exceeds the maximum allowed characters
                if queue_string.len() == 2000 {
                    break;
                }
            }

            queue_string.push_str(
                format!(
                    "{} songs in queue | Total length: `{}`",
                    queue_length - 1,
                    seconds_to_string(total_length).await
                )
                .as_str(),
            );
        }

        queue_embed.description(queue_string);
        check_msg(
            msg.channel_id
                .send_message(ctx, |m| {
                    m.set_embed(queue_embed);
                    m
                })
                .await,
        );
    }

    Ok(())
}

async fn get_track_info(
    track: &TrackHandle,
    map: &HashMap<Uuid, UserId>,
) -> (String, String, u64, String) {
    // Get the track title
    let title: String = match &track.metadata().title {
        Some(t) => t.clone(),
        None => "N/A".to_string(),
    };

    // Get the track url
    let url: String = match &track.metadata().source_url {
        Some(u) => u.clone(),
        None => "N/A".to_string(),
    };
    // Get the track duration
    let duration: u64 = match &track.metadata().duration {
        Some(d) => d.as_secs(),
        None => 0,
    };

    // Get who played the track
    let uuid = &track.uuid();
    let mention: String = match map.get(&uuid) {
        Some(userid) => format!("<@{}>", userid.0),
        None => "N/A".to_string(),
    };

    (title, url, duration, mention)
}
