

use serenity::{
    framework::standard::{
        macros::command, 
        CommandResult,
    },
    model::prelude::*,
    prelude::*,
};

use crate::util::{
    misc::check_msg,
    embeds,
};

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    check_msg(msg.channel_id.say(&ctx.http, "Pong!").await);

    Ok(())
}



#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    embeds::help(ctx, msg).await;
    Ok(())
}



#[command]
async fn todo(ctx: &Context, msg: &Message) -> CommandResult {
    check_msg(msg.channel_id.say(&ctx.http, "```prolog
TODO:

#Bugs
none known

#Commands
Aliases (show what aliases are available for each command)
Ping (more verbose ping)
Loop (loop the currently playing song)
Loopqueue (loop the whole queue)
Lyrics (get the lyrics of the currently playing song)
Removedupes (remove duplicate songs)
Leavecleanup (Remove absent users songs from the queue)

#Settings
Dj Only Mode (songs can onlt be played by those with the DJ role)
Announce Songs (bot announces each song as it comes up in the queue)
Blacklist Channels (blacklist certain channels from being used for music commands)
Blacklist / Whitelist Mode (switch between channel blacklist and whitelist)

#Internal
Implement a custom queue to improve the performance of queueing playlists
Add support for spotify links
Add support for soundcloud and spotify playlists
Manage Guild Is DJ (add a condition to the dj role check that returns true if the user has the manage guild perm)
Replace Placeholders (replace all placeholder messages with snazzy embeds)
```").await);

    Ok(())
}