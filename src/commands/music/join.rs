use crate::util::{embeds, music::MusicCommand};

use std::time::Duration;

use serenity::{
    async_trait,
    client::Context,
    framework::standard::{macros::command, CommandError, CommandResult},
    model::channel::Message,
};

use songbird::{driver::Bitrate::BitsPerSecond as Bits, Event, EventContext, EventHandler as VoiceEventHandler};

#[command]
#[only_in(guilds)]
#[aliases(connect, j)]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    join_fn(ctx, msg).await?;
    Ok(())
}

pub async fn join_fn(ctx: &Context, msg: &Message) -> Result<bool, CommandError> {
    let mc = MusicCommand::new(ctx, msg).await;

    // Get the channel the user is connected to
    let user_channel_id = mc.guild.voice_states.get(&msg.author.id).and_then(|voice_state| voice_state.channel_id);

    // Check if the bot is already in a voice chat
    if mc.guild.voice_states.get(&ctx.cache.current_user_id().await).is_some() {
        embeds::already_in_vc(ctx, msg).await;
        return Ok(false);
    }

    // Check if the user is actually in a voice channel or not
    let connect_to = match user_channel_id {
        Some(channel) => channel,
        None => {
            embeds::user_missing(ctx, msg).await;
            return Ok(false);
        }
    };

    // Get the songbird voice client and actually try to connect to the channel
    if let Some(manager) = mc.get_manager().await {
        let (handle_lock, success) = manager.join(mc.guild_id, connect_to).await;

        // Check if the connection was successful
        match success {
            Ok(_) => {
                embeds::connected(ctx, msg, connect_to).await;
                let mut handler = handle_lock.lock().await;
                handler.set_bitrate(Bits(384_000));
                handler.add_global_event(Event::Periodic(Duration::from_secs(600), None), ShouldLeaveChecker { ctx: ctx.clone(), msg: msg.clone(), channel_id: connect_to, manager: manager });
            }
            Err(_) => embeds::cannot_join(ctx, msg).await,
        }
    }

    Ok(true)
}



struct ShouldLeaveChecker {
    ctx: Context,
    msg: Message,
    channel_id: serenity::model::id::ChannelId,
    manager: std::sync::Arc<songbird::Songbird>,
}

#[async_trait]
impl VoiceEventHandler for ShouldLeaveChecker {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        let mc = MusicCommand::new(&self.ctx, &self.msg).await;

        let mut queue_empty = true;
        let mut channel_empty = true;
        let mut has_handler = false;

        if let Some(handler_lock) = self.manager.get(mc.guild_id) {
            let handler = handler_lock.lock().await;
            queue_empty = handler.queue().is_empty();
            has_handler = true;

            let channel = match mc.guild.channels.get(&self.channel_id) {
                Some(c) => c,
                None => { // If the channel doesn't exist, clean up
                    let _ = self.manager.remove(mc.guild_id).await;
                    mc.clear_votes().await;
                    mc.clear_utmap().await;
                    return None;
                },
            };

            match channel.members(&self.ctx.cache).await {
                Ok(m) => channel_empty = m.len() == 1, // 1 For the bot
                Err(_) => channel_empty = true,
            };
        }

        if queue_empty && has_handler {
            let _ = self.manager.remove(mc.guild_id).await;
            mc.clear_votes().await;
            mc.clear_utmap().await;
        }

        if channel_empty && has_handler {
            let _ = self.manager.remove(mc.guild_id).await;
            mc.clear_votes().await;
            mc.clear_utmap().await;
        }

        None
    }
}
