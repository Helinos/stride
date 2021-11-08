use crate::util::{embeds, misc::check_msg, music::MusicCommand};

use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};



#[command]
#[only_in(guilds)]
#[aliases(disconnect, dc, d, l)]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let mc = MusicCommand::new(ctx, msg).await;

    // Requires the DJ role or the Manage Guild permission
    if mc.has_dj().await && mc.connected().await.is_some() {
        let manager = mc.get_manager().await.unwrap();
        if let Err(e) = manager.remove(mc.guild_id).await {
            check_msg(msg.channel_id.say(&ctx.http, format!("Failed: {:?}", e)).await);
        }
        mc.clear_votes().await;
        mc.clear_utmap().await;
        embeds::bot_left(ctx, msg).await;
    }

    Ok(())
}
