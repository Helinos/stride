use lavalink_rs::player_context::QueueMessage;

use crate::{
    responses::{self, Say},
    Context, Error,
};

// Move a track from one position in the queue to another
#[poise::command(slash_command)]
pub async fn remove(
    context: Context<'_>,
    #[description = "The position of the track in the queue that te be removed from the queue."]
    position: usize,
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
    if 1 > position || position >= queue_length {
        responses::error(context, "There is no track at the specified ").await?;
        return Ok(());
    }

    let wrapped_track = queue.get(position - 1);
    
    let track = &wrapped_track.unwrap().track;
    let title = &track.info.title;

    match &track.info.uri {
        Some(uri) => responses::default(context, format!("Removed [{}]({}) from the queue.", title, uri)).await?,
        None => responses::default(context, format!("Removed {} from the queue.", title)).await?,
    }
    
    queue.remove(position - 1);
    player_context.set_queue(QueueMessage::Replace(queue))?;

    Ok(())
}
