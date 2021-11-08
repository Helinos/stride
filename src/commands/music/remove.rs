use crate::util::{
    music::MusicCommand,
    misc::check_msg,
    embeds,
};

use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};



#[command]
#[only_in(guilds)]
#[aliases(r)]
async fn remove (ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mc = MusicCommand::new(ctx, msg).await;
    let handler_lock = mc.connected().await;

    // Requires the DJ role or the Manage Guild permission
    if handler_lock.is_some() && mc.has_dj().await {
        let handler_lock = handler_lock.unwrap();
        let handler = handler_lock.lock().await;
        let queue = handler.queue();

        if queue.len() == 0 {
            embeds::not_playing(ctx, msg).await;
            return Ok(());
        }
        
        let index = match args.single::<usize>() {  
            Ok(0) => {
                embeds::index_oor(ctx, msg).await;
                return Ok(());
            }
            Ok(index) => index,
            Err(_) => {
                embeds::invalid_index(ctx, msg).await;
                return Ok(());
            }
        };
        
        match queue.dequeue(index) {
            Some(song) => {
                let title = match &song.metadata().title {
                    Some(title) => title,
                    None => "N/A"
                };
                check_msg(msg.channel_id.say(&ctx.http, format!("`Placeholder` Removed song {}: {}.", index, title)).await);
            },
            None => embeds::index_oor(ctx, msg).await,
        }
    }

    Ok(())
}