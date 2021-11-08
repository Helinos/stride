use crate::{
    util::{
        embeds,
        misc::{clean_title, seconds_to_string},
        music::MusicCommand,
    },
    UserTrackMap,
};

use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

#[command]
#[only_in(guilds)]
#[aliases(np, nowplaying)]
async fn now_playing(ctx: &Context, msg: &Message) -> CommandResult {
    let mc = MusicCommand::new(ctx, msg).await;
    let handler_lock = mc.connected().await;

    if handler_lock.is_some() {
        let handler_lock = handler_lock.unwrap();
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let current_track = match queue.current() {
            Some(current_track) => current_track,
            None => {
                embeds::not_playing(ctx, msg).await;
                return Ok(());
            }
        };

        // Get song title and format if necessary
        let mut title = match &current_track.metadata().title {
            Some(current_title) => current_title.to_string(),
            None => "N/A".to_string(),
        };

        title = clean_title(title).await;

        let url = match &current_track.metadata().source_url {
            Some(url) => url,
            None => "https://www.zombo.com/",
        };

        let data = ctx.data.read().await;
        let user_track_map_lock = data.get::<UserTrackMap>().expect("Expected UserTrackMap in TypeMap");
        let user_track_map = user_track_map_lock.read().await;
        let map = user_track_map.get(&mc.guild_id).unwrap();

        let uuid = &current_track.uuid();
        let mention: String = match map.get(&uuid) {
            Some(userid) => format!("<@{}>", userid.0),
            None => "N/A".to_string(),
        };

        let track_position = current_track.get_info().await?.position.as_secs();
        let track_duration = match current_track.metadata().duration {
            Some(track_duration) => track_duration.as_secs(),
            None => {
                embeds::track_error(ctx, msg).await;
                return Ok(());
            }
        };

        let mut progress_bar = "▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬".to_string();
        let max_characters = 20;
        let dot_position: usize = (max_characters as f64 * (track_position as f64 / track_duration as f64)) as usize;
        // progress_bar.replace_range(dot_position..dot_position + 1, "🔵");
        progress_bar.replace_range(progress_bar.char_indices().nth(dot_position).map(|(pos, ch)| (pos..pos + ch.len_utf8())).unwrap(), "🔵");

        let np_string = format!("[{}]({}) [{}]", title, url, mention);
        let np_bar = format!("{} {}/{}", progress_bar, seconds_to_string(track_position).await, seconds_to_string(track_duration).await);

        embeds::now_playing(ctx, msg, np_string, np_bar).await;
    }

    Ok(())
}
