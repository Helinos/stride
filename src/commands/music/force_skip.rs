use crate::util::{
    embeds,
    music::MusicCommand,
};

use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};



#[command]
#[only_in(guilds)]
#[aliases(fs, forceskip)]
async fn force_skip(ctx: &Context, msg: &Message) -> CommandResult {
    let music_command = MusicCommand::new(ctx, msg).await;
    let handler_lock = music_command.connected().await;
    
    // Requires the DJ role or the Manage Guild permission
    if handler_lock.is_some() && music_command.has_dj().await {
        let handler_lock = handler_lock.unwrap();
        let handler = handler_lock.lock().await;
        let queue = handler.queue();

        let position = handler.queue().len();
        if position > 0 {
            queue.skip()?;
            music_command.clear_votes().await;
            embeds::skipped(ctx, msg).await;
        } else {
            embeds::not_playing(ctx, msg).await;
        }
    }
    
    Ok(())
}