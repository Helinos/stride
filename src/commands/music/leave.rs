use crate::{
    Context, Error, responses::{self, Say},
};

/// Disconnect the bot from the current channel.
#[poise::command(slash_command)]
pub async fn leave(
    context: Context<'_>,
) -> Result<(), Error> {
    let guild = context.guild().unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(context.serenity_context())
                .await
                .unwrap()
                .clone();

    let lava_client = context.data().lavalink.clone();

    let Some(_player_context) = lava_client.get_player_context(guild_id) else {
        responses::ErrorMessage::BotNotInVC.say(context).await?;
        return Ok(());
    };

    manager.leave(guild_id).await?;
    lava_client.delete_player(guild_id).await?;

    responses::default(context, format!("Left the voice channel.")).await?;

    return Ok(());
}