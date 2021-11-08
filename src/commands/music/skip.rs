use crate::{
    GuildSkipVotes, 
    util::{
        music::MusicCommand,
        misc::check_msg,
        embeds,
    },
};

use std::collections::HashSet;

use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};



#[command]
#[only_in(guilds)]
#[aliases(s, voteskip)]
async fn skip(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {    
    let mc = MusicCommand::new(ctx, msg).await;
    let handler_lock = mc.connected().await;

    if handler_lock.is_some() {
        let handler_lock = handler_lock.unwrap();
        let handler = handler_lock.lock().await;   
        let queue = handler.queue();

        let position = handler.queue().len();
        if position > 0 {

            let data = ctx.data.read().await;
            let guild_skip_votes_lock = data.get::<GuildSkipVotes>().expect("Expected GuildSkipVotes in TypeMap");
            let mut guild_skip_votes = guild_skip_votes_lock.write().await;
            
            // Check if the guild has a list of voters,
            // otherwise make a new one and add the user to the list
            let has_voters = guild_skip_votes.get(&mc.guild_id);
            let already_voted: bool;
            if let Some(voters) = has_voters {
                // Add the user to the list of voters if they aren't already present
                if voters.contains(&msg.author.id) {
                    already_voted = true;
                } else {
                    let mut voters = voters.clone();
                    let _ = voters.insert(msg.author.id);
                    guild_skip_votes.insert(mc.guild_id, voters);
                    already_voted = false;
                }
            } else {
                let mut voters = HashSet::new();
                let _ = voters.insert(msg.author.id);
                guild_skip_votes.insert(mc.guild_id, voters);
                already_voted = false;
            }

            
            // Get the percentage of poeple that have voted.
            let guild = msg.guild(&ctx.cache).await.unwrap();
            let voters = guild_skip_votes.get(&mc.guild_id).unwrap();
            let mut votes = 0;
            let mut vc_members = 0;
            // Get the state of each user in a voice channel
            for (user_id, voice_state) in guild.voice_states.iter() {
                // If the user is in the same channel as the bot, 
                if voice_state.channel_id.unwrap().0 == handler.current_channel().unwrap().0 {
                    vc_members += 1;
                    if voters.contains(user_id) {
                        votes += 1;
                    } 
                }
            }

            let needed_votes = ((vc_members - 1) as f64 / 2.0).ceil() as u64; // Subtract 1 from vc_members to counteract the bot

            // If there aren't enough votes yet, just print a message
            if needed_votes > votes {
                if already_voted {
                    check_msg(msg.channel_id.say(&ctx.http, format!("`Placeholder` You already voted {}/{} People", votes, needed_votes)).await);
                } else {
                    check_msg(msg.channel_id.say(&ctx.http, format!("`Placeholder` Skipping {}/{} People", votes, needed_votes)).await);
                }
                return Ok(())
            } else { // If there are enough votes, skip
                queue.skip()?;
                guild_skip_votes.remove(&mc.guild_id);
                embeds::skipped(ctx, msg).await;
            }

        } else {
            embeds::not_playing(ctx, msg).await;
        }
    }

    Ok(())
}