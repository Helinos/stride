use std::collections::VecDeque;

use crate::{
    responses::{self, Say},
    Context, Error, commands::music::millis_to_string,
};
use lavalink_rs::{prelude::{SearchEngines, TrackInQueue, TrackLoadData}, player_context::QueueMessage};
use tracing::info;

/// Play a song in the voice channel you are connected to.
#[poise::command(slash_command)]
pub async fn play(
    context: Context<'_>,
    #[description = "Search term or URL"] query: String,
) -> Result<(), Error> {
    let guild = context.guild().unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(context.serenity_context())
        .await
        .unwrap()
        .clone();
    let lava_client = context.data().lavalink.clone();

    if manager.get(guild_id).is_none() || lava_client.get_player_context(guild_id).is_none() {
        // Get the channel the author of the command is in
        let channel_id = guild
            .voice_states
            .get(&context.author().id)
            .and_then(|voice_state| voice_state.channel_id);

        let connect_to = match channel_id {
            Some(channel) => channel,
            None => {
                responses::ErrorMessage::UserMissingFromVC
                    .say(context)
                    .await?;
                return Ok(());
            }
        };

        // Connect to the voice channel
        let (_, handler) = manager.join_gateway(guild_id, connect_to).await;

        match handler {
            Ok(connection_info) => {
                lava_client
                    .create_player_context(guild_id, connection_info)
                    .await?;
            }
            Err(why) => {
                responses::ErrorMessage::BotCannotJoinVC(why)
                    .say(context)
                    .await?;
                return Ok(());
            }
        }
    }

    // This will only happen if someone disconnects the bot milliseconds after playing a song. I assume.
    let Some(player_context) = lava_client.get_player_context(guild_id) else {
        responses::ErrorMessage::BotNotInVC.say(context).await?;
        return Ok(());
    };

    let query = if query.starts_with("http") {
        query
    } else {
        SearchEngines::YouTube.to_query(&query)?
    };

    info!("query: {:?}", query);

    let loaded_tracks = lava_client.load_tracks(guild_id, &query).await?;

    let mut playlist_info = None;
    let mut playlist_count = None;

    let tracks: VecDeque<TrackInQueue> = match loaded_tracks.data {
        Some(TrackLoadData::Track(track)) => VecDeque::from([track.into()]),
        Some(TrackLoadData::Search(search_results)) => VecDeque::from([search_results[0].clone().into()]),
        Some(TrackLoadData::Playlist(playlist)) => {
            playlist_info = Some(playlist.info);
            playlist_count = Some(playlist.tracks.len());
            playlist.tracks.iter().map(|track| track.into()).collect()
        }

        _ => {
            context.say(format!("Something went horribly wrong. Here's some dubiously relevant data: {:?}", loaded_tracks)).await?;
            return Ok(());
        }
    };

    if let Some(info) = playlist_info {
        responses::default(
            context,
            format!(
                "Queued **{}** tracks from playlist: [{}]({})",
                playlist_count.unwrap(),
                info.name,
                query
            ),
        )
        .await?;
    } else {
        let track = &tracks[0].track;

        let track_length = millis_to_string(track.info.length);

        let queue_length = if player_context.get_player().await?.track.is_some() {
            player_context.get_queue().await?.len() + 1
        } else {
            0
        };

        if queue_length == 0 {
            // If the queue is empty and there is a url
            if let Some(uri) = &track.info.uri {
                responses::default(
                    context,
                    format!(
                        "Started playing `{}` [{}]({}) - [<@{}>]",
                        track_length,
                        track.info.title,
                        uri,
                        context.author().id
                    ),
                )
                .await?;
            // If the queue is empty and there isn't a url
            } else {
                responses::default(
                    context,
                    format!(
                        "Started playing `{}` {} - [<@{}>]",
                        track_length,
                        track.info.title,
                        context.author().id
                    ),
                )
                .await?;
            }
        } else {
            // If there is a queue and there is a url
            if let Some(uri) = &track.info.uri {
                responses::default(
                    context,
                    format!(
                        "Queued at position {} `{}` [{}]({}) - [<@{}>]",
                        queue_length,
                        track_length,
                        track.info.title,
                        uri,
                        context.author().id
                    ),
                )
                .await?;
            // If there is a queue and there isn't a url
            } else {
                responses::default(
                    context,
                    format!(
                        "Queued at position {} `{}` {} - [<@{}>]",
                        queue_length,
                        track_length,
                        track.info.title,
                        context.author().id
                    ),
                )
                .await?;
            }
        }
    }

    player_context.set_queue(QueueMessage::Append(tracks))?;

    // We need to skip if there's nothing currently playing? I got this from the example
    if let Ok(player_data) = player_context.get_player().await {
        if player_data.track.is_none() && player_context.get_queue().await.is_ok_and(|queue| !queue.is_empty()) {
            player_context.skip()?;
        }
    }

    Ok(())
}
