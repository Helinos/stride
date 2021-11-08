use crate::util::{
    embeds,
    music::MusicCommand,
};

use serenity::{
    client::Context,
    framework::standard::{
        macros::command, 
        CommandResult,
    },
    model::channel::Message,
};



#[command]
#[only_in(guilds)]
#[aliases(c, stop)]
async fn clear (ctx: &Context, msg: &Message) -> CommandResult {
    let mc = MusicCommand::new(ctx, msg).await;
    let handler_lock = mc.connected().await;

    // Requires the DJ role or the Manage Guild permission
    if handler_lock.is_some() && mc.has_dj().await {
        let handler_lock = handler_lock.unwrap();
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        
        let position = handler.queue().len();
        if position > 0 {
            queue.stop();
            mc.clear_votes().await;
            mc.clear_utmap().await;
            embeds::queue_cleared(ctx, msg).await;
        } else {
            embeds::not_playing(ctx, msg).await;
        }
    }

    Ok(())
}