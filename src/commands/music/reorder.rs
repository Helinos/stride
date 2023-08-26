use crate::{
    responses::{self, Say},
    Context, Error,
};

// Move a track from one position in the queue to another
#[poise::command(slash_command)]
pub async fn reorder(
    context: Context<'_>,
    #[description = "The position of the track in the queue that you want to move."]
    #[rename = "from"]
    position_from: usize,
    #[description = "The position that you want to move the track to."]
    #[rename = "to"]
    position_to: usize
) -> Result<(), Error> {
    let guild = context.guild().unwrap();
    let guild_id = guild.id;

    let lava_client = context.data().lavalink.clone();

    let Some(player_context) = lava_client.get_player_context(guild_id) else {
        responses::ErrorMessage::BotNotInVC.say(context).await?;
        return Ok(());
    };

    if player_context.get_player().await?.track.is_none() {
        responses::ErrorMessage::BotNotPlaying.say(context).await?;
        return Ok(());
    }

    let mut queue = player_context.get_queue().await?;
    let queue_length = queue.len();

    // Test if the either position is valid
    if 1 > position_from || position_from >= queue_length || 1 > position_to || position_to > queue_length  {
        responses::ErrorMessage::InvalidMove.say(context).await?;
        return Ok(());
    }

    let index_from = position_from - 1;
    let index_to = position_to - 1;

    let wrapped_track = queue.remove(index_from);

    match wrapped_track {
        Some(wrapped_track) => {
            let track = &wrapped_track.track;
            
            match &track.info.uri {
                Some(uri) => responses::default(context, format!("Moved [{} - {}]({}) from position {} to position {}.", track.info.author, track.info.title, uri, position_from, position_to)).await?,
                None => responses::default(context, format!("Moved {} - {} from position {} to position {}.", track.info.author, track.info.title, position_from, position_to)).await?,
            }

            queue.insert(index_to, wrapped_track);
            player_context.replace_queue(queue)?;
        },
        None => {
            responses::error(context, "Tried to remove a track the queue at an invalid index. (This should never happen!)").await?;
        }
    }

    Ok(())
}
