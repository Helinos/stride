use crate::{
    util::{
        embeds,
        music::{MusicCommand, ytdl_playlist},
    },
    UserTrackMap,
};

use super::join::join_fn;

use std::collections::{HashMap, VecDeque};

use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use songbird::{input::restartable::Restartable, tracks::Queued};



// #[command]
// #[only_in(guilds)]
// async fn test (ctx: &Context, msg: &Message) -> CommandResult {
//     let source = match ytdl_playlist("https://www.youtube.com/playlist?list=PLVYcJc38W3P2GWVRVR8LlQwxRSYeJ69B-").await {
//         Ok(source) => source,
//         Err(why) => {
//             println!("Err getting source: {:?}", why);
//             // embeds::url_error(ctx, msg).await;
//             return Ok(());
//         }
//     };
//     check_msg(msg.channel_id.say(&ctx.http, format!("{}", source.len())).await);
//     Ok(())
// }



#[command]
#[only_in(guilds)]
#[aliases(p)]
async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let (Some(handler_lock), playlist_len) = play_fn(ctx, msg, args.message().to_string(), false).await {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let queue_length = queue.len();

        if playlist_len.is_none() {
            // If this is the first song being added, don't say it's been "added to a queue"
            if queue_length == 1 {
                embeds::playing_song(ctx, msg, queue).await;
            } else {
                embeds::queued(ctx, msg, queue).await;
            }
        } else {
            embeds::queued_multiple(ctx, msg, playlist_len.unwrap()).await;
        }
    }

    Ok(())
}



#[command]
#[only_in(guilds)]
#[aliases(pt, playtop, playnext, pn)]
async fn play_top(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let (Some(handler_lock), playlist_len) = play_fn(ctx, msg, args.message().to_string(), true).await {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let queue_length = queue.len();

        if playlist_len.is_none() {
            match queue_length {
                // 0 and 1 are conditions where !playtop funtions just like !play
                0 => embeds::playing_song(ctx, msg, queue).await,
                1 => embeds::queued_top(ctx, msg, queue).await, 
                _ => {
                    let last_to_second = |q: &mut VecDeque<Queued>| {
                        let last = q.pop_back().unwrap();
                        q.insert(1, last);
                    };

                    queue.modify_queue(last_to_second);
                    embeds::queued_top(ctx, msg, queue).await;
                }
            }
        } else {
            // 0 and 1 are conditions where !playtop funtions just like !play
            if queue_length != 0 || queue_length != 1 {
                let move_pl_top = |q: &mut VecDeque<Queued>| {
                    let split_pos = queue_length - playlist_len.unwrap();
                    let mut new_songs = q.split_off(split_pos);
                    for _ in 0..new_songs.len() {
                        q.insert(1, new_songs.pop_back().unwrap());
                    }
                };

                queue.modify_queue(move_pl_top);
            }
            embeds::queued_multiple(ctx, msg, playlist_len.unwrap()).await;
        }
    }

    Ok(())
}



#[command]
#[only_in(guilds)]
#[aliases(ps, pn, playskip, playnow)]
async fn play_skip(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let (Some(handler_lock), playlist_len) = play_fn(ctx, msg, args.message().to_string(), true).await {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let queue_length = queue.len();
        if playlist_len.is_none() {
            match queue_length {
                0 => (), // If there's no songs in the queue nothing needs to be done
                1 => queue.skip()?, // If there's only the song being played all we need to do is skip it
                _ => {                              
                    let last_to_second = |queue: &mut VecDeque<Queued>| {
                        let last = queue.pop_back().unwrap();
                        queue.insert(1, last);
                    };

                    queue.modify_queue(last_to_second);
                    queue.skip()?
                }          
            }
            embeds::playing_song(ctx, msg, queue).await;
        } else {
            match queue_length {
                0 => (),
                1 => queue.skip()?,
                _ => {
                    let move_pl_top = |q: &mut VecDeque<Queued>| {
                        let split_pos = queue_length - playlist_len.unwrap();
                        let mut new_songs = q.split_off(split_pos);
                        for _ in 0..new_songs.len() {
                            q.insert(1, new_songs.pop_back().unwrap());
                        }
                    };
    
                    queue.modify_queue(move_pl_top);
                    queue.skip()?;
                }
            }
            embeds::queued_multiple(ctx, msg, playlist_len.unwrap()).await
        }

        
    }

    Ok(())
}



async fn play_fn(ctx: &Context, msg: &Message, query: String, dj: bool) -> (Option<std::sync::Arc<serenity::prelude::Mutex<songbird::Call>>>, Option<usize>) {
    let mc = MusicCommand::new(ctx, msg).await;
    
    // See if the user needs the dj role
    if dj && !mc.has_dj().await {
        return (None, None);
    }

    // Get the guild and the manager and see if the user if in a voice channel
    if let Some(manager) = mc.get_manager().await {
        let mut possible_handler = manager.get(mc.guild_id);
        let has_handler = possible_handler.is_some();
        
        // See if the bot isn't in a voice channel and join a voice channel if it isn't
        if !has_handler {
            // check_msg(msg.channel_id.say(&ctx.http, "`Placeholder` Bot is not in a voice channel.").await,);
            match join_fn(ctx, msg).await {
                Ok(true) => possible_handler = manager.get(mc.guild_id),
                _ => return (None, None) // Error handled by join_fn, but we don't need to proceed pass this point if there was an error
            }
        }

        if let Some(handler_lock) = possible_handler {
            let mut handler = handler_lock.lock().await;
            // See if the bot isn't in a voice channel even though it has a handler
            if None == handler.current_connection() {
                drop(handler); // Drop the lock so the bot can join a voice channel
                match join_fn(ctx, msg).await {
                    Ok(true) => handler = handler_lock.lock().await,
                    _ => return (None, None),
                }
            }

            // Here, we use lazy restartable sources to make sure that we don't pay
            // for decoding, playback on tracks which aren't actually live yet.
            let mut track_count = None;

            if &query == "" { // If there is no query
                embeds::url_missing(ctx, msg).await;
                return (None, None);
            } else if query.starts_with("http") { // If the query is a link
                // If the query is a youtube playlist
                if query.contains("list=") && query.contains("youtube.com") { 
                    let links = match ytdl_playlist(query).await {
                        Ok(links) => links,
                        Err(why) => {
                            println!("Err getting links from playlist {:?}", why);
                            embeds::url_error(ctx, msg).await;
                            return (None, None);
                        }
                    };

                    let mut count = 0;
                    for link in links.iter() {
                        let source = match Restartable::ytdl(link.clone(), true).await {
                            Ok(source) => {
                                count += 1;
                                source
                            }
                            Err(_) => continue,
                        };
                        enqueue(&mc, &mut handler, source).await;
                    }
                    track_count = Some(count);

                // If the query is any link
                } else { 
                    let source = match Restartable::ytdl(query, true).await {
                        Ok(source) => source,
                        Err(why) => {
                            println!("Err starting source: {:?}", why);
                            embeds::url_error(ctx, msg).await;
                            return (None, None);
                        }
                    };
                    enqueue(&mc, &mut handler, source).await;
                }
            // If the query is assumed to be a search    
            } else {
                let source = match Restartable::ytdl_search(query, true).await {
                    Ok(source) => source,
                    Err(why) => {
                        println!("Err starting source: {:?}", why);
                        embeds::search_error(ctx, msg).await;
                        return (None, None);
                    }
                };
                enqueue(&mc, &mut handler, source).await;
            }

            drop(handler);
            return (Some(handler_lock), track_count);
        }
    }

    (None, None)
}

async fn enqueue (mc: &MusicCommand<'_>, handler: &mut tokio::sync::MutexGuard<'_, songbird::Call>, source: Restartable) {
    handler.enqueue_source(source.into());

    let queue = handler.queue();
    let queue_length = queue.len();
    let latest_track_uuid = queue.current_queue().get(queue_length - 1).unwrap().uuid();
    let requester_id = mc.msg.author.id;

    let data = mc.ctx.data.read().await;
    let user_track_map_lock = data.get::<UserTrackMap>().expect("Expected UserTrackMap in TypeMap");
    let mut user_track_map = user_track_map_lock.write().await;

    let has_map = user_track_map.get(&mc.guild_id);
    if let Some(map) = has_map {
        let mut map = map.clone();
        let _ = map.insert(latest_track_uuid, requester_id);
        user_track_map.insert(mc.guild_id, map);
    } else {
        let mut map = HashMap::new();
        let _ = map.insert(latest_track_uuid, requester_id);
        user_track_map.insert(mc.guild_id, map);
    }
}
