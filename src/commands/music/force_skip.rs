use lavalink_rs::player_context::QueueMessage;

use crate::{
    responses::{self, Say},
    Context, Error,
};

/// Skip a track without voting
#[poise::command(slash_command, rename = "forceskip")]
pub async fn force_skip(
    context: Context<'_>,
    #[description = "The position in the queue to skip to"] 
    position: Option<usize>,
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

    match position {
        Some(position) => {
            if 1 > position || position > queue_length {
                responses::ErrorMessage::InvalidSkip.say(context).await?;
                return Ok(());
            }

            let new_queue = queue.split_off(position as usize - 1);
            player_context.set_queue(QueueMessage::Replace(new_queue))?;

            responses::DefaultMessage::SkippedTo(position).say(context).await?;
        }
        None => responses::DefaultMessage::Skipped.say(context).await?,
    }
    
    player_context.skip()?;

    Ok(())
}
