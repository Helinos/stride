use crate::util::{
    music::MusicCommand,
    misc::check_msg,
    embeds,
};

use std::collections::VecDeque;

use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use songbird::tracks::Queued;



#[command]
#[only_in(guilds)]
#[aliases(m, move)]
async fn reorder(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mc = MusicCommand::new(ctx, msg).await;
    let handler_lock = mc.connected().await;

    // Requires the DJ role or the Manage Guild permission
    if handler_lock.is_some() && mc.has_dj().await {
        let handler_lock = handler_lock.unwrap();
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
    
        let queue_length = queue.len();
        match queue_length {
            0 => embeds::not_playing(ctx, msg).await,
            1 => check_msg(msg.channel_id.say(&ctx.http, "`Placeholder` There is only one song.").await),
            _ => {

                let first_index = match args.single::<usize>() {  
                    Ok(0) => {
                        check_msg(msg.channel_id.say(&ctx.http, "`Placeholder` First index out of range.").await);
                        return Ok(());
                    }
                    Ok(first_index) => first_index,
                    Err(_) => {
                        check_msg(msg.channel_id.say(&ctx.http, "`Placeholder` First index invalid.").await);
                        return Ok(());
                    }
                };

                let second_index = match args.single::<usize>() {  
                    Ok(0) => {
                        check_msg(msg.channel_id.say(&ctx.http, "`Placeholder` Second index out of range.").await);
                        return Ok(());
                    }
                    Ok(second_index) => second_index,
                    Err(_) => {
                        check_msg(msg.channel_id.say(&ctx.http, "`Placeholder` Second index invalid.").await);
                        return Ok(());
                    }
                };

                if first_index == second_index {
                    check_msg(msg.channel_id.say(&ctx.http, "`Placeholder` Cannot replace a song with itself.").await);
                    return Ok(());
                }

                if first_index >= queue_length || second_index >= queue_length {
                    embeds::index_oor(ctx, msg).await;
                    return Ok(());
                }

                check_msg(msg.channel_id.say(&ctx.http, format!("`Placeholder` Moved song {} to position {}.", first_index, second_index)).await);

                // if first_index < second_index {
                //     second_index = second_index - 1;
                // }

                let song = match queue.dequeue(first_index) {
                    Some(song) => song,
                    None => {
                        check_msg(msg.channel_id.say(&ctx.http, "`Placeholder` Error moving song.").await);
                        return Ok(())
                    }
                };
                
                let add = |q: &mut VecDeque<Queued>| {
                    q.insert(second_index, song);
                };

                queue.modify_queue(add);

            }
        }
    }

    Ok(())
}