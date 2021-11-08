use crate::util::{
    embeds,
    music::MusicCommand,
};

use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};



#[command]
#[only_in(guilds)]
#[aliases(skipto, forceskipto)]
async fn force_skip_to(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mc = MusicCommand::new(ctx, msg).await;
    let handler_lock = mc.connected().await;

    if handler_lock.is_some() && mc.has_dj().await {
        let handler_lock = handler_lock.unwrap();
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let queue_length = queue.len();

        if queue_length == 0 {
            embeds::not_playing(ctx, msg).await;
            return Ok(());
        }

        // Check for invalid indexs as well as the index of 1
        let index = match args.single::<usize>() {  
            Ok(0) => {
                embeds::index_oor(ctx, msg).await;
                return Ok(());
            },
            Ok(1) => {
                queue.skip()?;
                mc.clear_votes().await;
                embeds::skipped(ctx, msg).await;
                return Ok(());
            },
            Ok(index) => index,
            Err(_) => {
                embeds::invalid_index(ctx, msg).await;
                return Ok(());
            }
        };

        if index > queue_length - 1 {
            embeds::index_oor(ctx, msg).await;
            return Ok(());
        }

        for _ in 1..index {
            queue.dequeue(1).unwrap();
        }

        embeds::skipped_to(ctx, msg, index).await;
        queue.skip()?;
    }
    
    Ok(())
}