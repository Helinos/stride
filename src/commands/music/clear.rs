use lavalink_rs::player_context;

use crate::{
    Context, Error, responses::{self, Say},
};

/// Clear the queue, optionally inculding the currently playing audio.
#[poise::command(slash_command)]
pub async fn clear(
    context: Context<'_>,
    #[description = "Whether to stop and clear the currently playing audio."] now_playing: Option<bool>,
) -> Result<(), Error> {
    let guild = context.guild().unwrap();
    let guild_id = guild.id;

    let lava_client = context.data().lavalink.clone();

    let Some(player_context) = lava_client.get_player_context(guild_id) else {
        responses::ErrorMessage::BotNotInVC.say(context).await?;
        return Ok(());
    };

    player_context.set_queue(player_context::QueueMessage::Clear)?;

    if now_playing.unwrap_or(false) {
        player_context.skip()?;
    }

    responses::default(context, format!("Cleared the queue.")).await?;

    return Ok(());
}