use crate::util::{embeds, misc, music::MusicCommand};

use rand::thread_rng;
use std::collections::VecDeque;

use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use songbird::tracks::Queued;

#[command]
#[only_in(guilds)]
async fn shuffle(ctx: &Context, msg: &Message) -> CommandResult {
    let mc = MusicCommand::new(ctx, msg).await;
    let handler_lock = mc.connected().await;

    // Requires the DJ role or the Manage Guild permission
    if handler_lock.is_some() && mc.has_dj().await {
        let handler_lock = handler_lock.unwrap();
        let handler = handler_lock.lock().await;
        let queue = handler.queue();

        match queue.len() {
            0 => {
                embeds::not_playing(ctx, msg).await;
                return Ok(());
            }
            1 => {
                embeds::cant_shuffle_0(ctx, msg).await;
                return Ok(());
            }
            2 => {
                embeds::cant_shuffle_1(ctx, msg).await;
                return Ok(());
            }
            _ => (),
        }

        let first_song = queue.dequeue(0).unwrap();

        let s = |q: &mut VecDeque<Queued>| {
            misc::shuffle(q, thread_rng());
            q.push_front(first_song);
        };
        queue.modify_queue(s);
        embeds::shuffled(ctx, msg).await;
    }

    Ok(())
}
