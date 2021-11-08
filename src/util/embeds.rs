use serenity::{
    utils::Color,
    model::{
        prelude::*,
        guild::Role,
        id::ChannelId,
    },
    prelude::*,
};

use songbird::tracks::TrackQueue;

use super::misc::{check_msg, clean_title};



const DEFAULT_COLOR: Color = Color::from_rgb(149, 165, 166);
const ERROR_COLOR: Color = Color::from_rgb(231, 76, 60);
const SETTINGS_COLOR: Color = Color::from_rgb(13, 71, 161);



/// =====================
/// 
///     META MESSAGES
/// 
/// =====================



pub async fn help (ctx: &Context, msg: &Message) {
    check_msg(msg.channel_id.send_message(ctx, |m| m.embed(|e| {
        e.title("Commands");
        e.color(DEFAULT_COLOR);
        e.description("System
        - **Help** Show this message.
        - **Ping** Pong!
        - **Settings** Change how the bot behaves in this server.
        - **Todo** See a list of features that are still in progress
    
        Music
        - **Join** Summon the bot to the voice channel you're in.
        - **Leave** *(DJ only)* Disconnect the bot from the voice channel it's in.
        - **Play <query>** Add a song from a name or url.
        - **Playtop <query>** *(DJ only)* Add a song directly to the top of the queue.
        - **Playskip <query>** *(DJ only)* Skip the current song and plays the requested song.
        - **Skip** Vote to skip on the currently playing song.
        - **Forceskip** *(DJ only)* Skip the currently playing song without voiting.
        - **Skipto <index>** *(DJ only)* Skip to a song at a specified index.
        - **Remove <index>** *(DJ only)* Remove a song from the queue at a given index.
        - **Move <index> <index>** *(DJ only)* Move a song from one index to the other.
        - **Queue** View the next 10 songs in the queue.
        - **Clear** *(DJ only)* Clear the queue.
        - **Nowplaying** See the title of the song that's currently being played.");
        e
  })).await);
}

pub async fn source (ctx: &Context, msg: &Message) {
    check_msg(msg.channel_id.send_message(ctx, |m| {
        m.reference_message(msg);
        m.embed(|e| {
            e.color(DEFAULT_COLOR);
            e.description("[Stride on github](https://github.com/DontStarve72/stride)");
            e
        });
        m
    }).await);
}



/// =======================
/// 
///     SYSTEM MESSAGES
/// 
/// =======================



pub async fn playing_song (ctx: &Context, msg: &Message, queue: &TrackQueue) {
    let track = queue.current().unwrap();
    
    let mut title = match &track.metadata().title {
        Some(title) => title.to_string(),
        None => "N/A".to_string(),
    };

    title = clean_title(title).await;

    let url = match &track.metadata().source_url {
        Some(url) => url,
        None => "https://www.zombo.com/",
    };

    println!("{}", title);

    check_msg(msg.channel_id.send_message(ctx, |m| {
        m.reference_message(msg);
        m.embed(|e| {
            e.color(DEFAULT_COLOR);
            e.description(format!("Playing [{}]({}) [<@{}>]", title, url, msg.author.id.0));
            e
        });
        m
    }).await);
}

pub async fn queued_top (ctx: &Context, msg: &Message, queue: &TrackQueue) {
    let q = queue.current_queue();
    let track = q.get(1).unwrap();
    
    let mut title = match &track.metadata().title {
        Some(title) => title.to_string(),
        None => "N/A".to_string(),
    };

    title = clean_title(title).await;

    let url = match &track.metadata().source_url {
        Some(url) => url,
        None => "https://www.zombo.com/",
    };

    check_msg(msg.channel_id.send_message(ctx, |m| {
        m.reference_message(msg);
        m.embed(|e| {
            e.color(DEFAULT_COLOR);
            e.description(format!("Queued `Position 1` [{}]({}) [<@{}>]", title, url, msg.author.id.0));
            e
        });
        m
    }).await);
}

pub async fn queued (ctx: &Context, msg: &Message, queue: &TrackQueue) {
    let queue_length = queue.len();
    let q = queue.current_queue();
    let track = q.get(queue_length - 1).unwrap();
    
    let mut title = match &track.metadata().title {
        Some(title) => title.to_string(),
        None => "N/A".to_string(),
    };

    title = clean_title(title).await;

    let url = match &track.metadata().source_url {
        Some(url) => url,
        None => "https://www.zombo.com/",
    };

    check_msg(msg.channel_id.send_message(ctx, |m| {
        m.reference_message(msg);
        m.embed(|e| {
            e.color(DEFAULT_COLOR);
            e.description(format!("Queued `Position {}`  [{}]({}) [<@{}>]", queue_length - 1, title, url, msg.author.id.0));
            e
        });
        m
    }).await);
        
}

pub async fn now_playing (ctx: &Context, msg: &Message, np_string: String, np_bar: String) {
    check_msg(msg.channel_id.send_message(ctx, |m| {
        m.reference_message(msg);
        m.embed(|e| {
            e.color(DEFAULT_COLOR);
            e.description(format!("{}", np_string));
            e.footer(|f| {
                f.text(format!("{}", np_bar));
                f
            });
            e
        });
        m
    }).await);
}

pub async fn queued_multiple (ctx: &Context, msg: &Message, count: usize) {
    check_msg(msg.channel_id.send_message(ctx, |m| {
        m.reference_message(msg);
        m.embed(|e| {
            e.color(DEFAULT_COLOR);
            e.description(format!("Queued **{}** tracks.", count));
            e
        });
        m
    }).await);
}

pub async fn connected (ctx: &Context, msg: &Message, channel: ChannelId) {
    check_msg(msg.channel_id.send_message(ctx, |m| {
        m.reference_message(msg);
        m.embed(|e| {
            e.color(DEFAULT_COLOR);
            e.description(format!("Joined {}", channel.mention()));
            e
        });
        m
    }).await);
}

pub async fn skipped_to (ctx: &Context, msg: &Message, amount: usize) {
    check_msg(msg.channel_id.send_message(ctx, |m| {
        m.reference_message(msg);
        m.embed(|e| {
            e.color(DEFAULT_COLOR);
            e.description(format!("Skipped {} songs.", amount));
            e
        });
        m
    }).await);
}

pub async fn skipped (ctx: &Context, msg: &Message) {
    check_msg(msg.channel_id.send_message(ctx, |m| {
        m.reference_message(msg);
        m.embed(|e| {
            e.color(DEFAULT_COLOR);
            e.description("Song skipped.");
            e
        });
        m
    }).await);
}

pub async fn queue_cleared (ctx: &Context, msg: &Message) {
    check_msg(msg.channel_id.send_message(ctx, |m| {
        m.reference_message(msg);
        m.embed(|e| {
            e.color(DEFAULT_COLOR);
            e.description("Queue cleared.");
            e
        });
        m
    }).await);
}

pub async fn bot_left (ctx: &Context, msg: &Message) {
    check_msg(msg.channel_id.send_message(ctx, |m| {
        m.reference_message(msg);
        m.embed(|e| {
            e.color(DEFAULT_COLOR);
            e.description("Left the voice channel.");
            e
        });
        m
    }).await);
}

pub async fn shuffled (ctx: &Context, msg: &Message) {
    check_msg(msg.channel_id.send_message(ctx, |m| {
        m.reference_message(msg);
        m.embed(|e| {
            e.color(DEFAULT_COLOR);
            e.description("Shuffled the queue.");
            e
        });
        m
    }).await);
}



/// ======================
/// 
///     ERROR MESSAGES 
/// 
/// ======================



pub async fn need_dj (ctx: &Context, msg: &Message, dj_role: &Role) {
    check_msg(msg.channel_id.send_message(ctx, |m| {
        m.reference_message(msg);
        m.embed(|e| {
            e.color(ERROR_COLOR);
            e.description(format!("You need the the <@&{}> role to run this command.", dj_role.id.0));
            e
        });
        m
    }).await);
}

pub async fn no_dj (ctx: &Context, msg: &Message) {
    check_msg(msg.channel_id.send_message(ctx, |m| {
        m.reference_message(msg);
        m.embed(|e| {
            e.color(ERROR_COLOR);
            e.description("You need the a `DJ` role to run this command, but no such role has been configured.");
            e
        });
        m
    }).await);
}

pub async fn bot_missing (ctx: &Context, msg: &Message) {
    check_msg(msg.channel_id.send_message(ctx, |m| {
        m.reference_message(msg);
        m.embed(|e| {
            e.color(ERROR_COLOR);
            e.description("The bot is not connected to a voice channel.");
            e
        });
        m
    }).await);
}

pub async fn not_playing (ctx: &Context, msg: &Message) {
    check_msg(msg.channel_id.send_message(ctx, |m| {
        m.reference_message(msg);
        m.embed(|e| {
            e.color(ERROR_COLOR);
            e.description("The bot is not currently playing anything.");
            e
        });
        m
    }).await);
}

pub async fn user_missing (ctx: &Context, msg: &Message) {
    check_msg(msg.channel_id.send_message(ctx, |m| {
        m.reference_message(msg);
        m.embed(|e| {
            e.color(ERROR_COLOR);
            e.description("You are not in any voice channels.");
            e
        });
        m
    }).await);
}

pub async fn cannot_join (ctx: &Context, msg: &Message) {
    check_msg(msg.channel_id.send_message(ctx, |m| {
        m.reference_message(msg);
        m.embed(|e| {
            e.color(ERROR_COLOR);
            e.description("Could not join the voice channel. (Insufficient permissions?).");
            e
        });
        m
    }).await);
}

pub async fn index_oor (ctx: &Context, msg: &Message) {
    check_msg(msg.channel_id.send_message(ctx, |m| {
        m.reference_message(msg);
        m.embed(|e| {
            e.color(ERROR_COLOR);
            e.description("Index out of range.");
            e
        });
        m
    }).await);
}

pub async fn invalid_index (ctx: &Context, msg: &Message) {
    check_msg(msg.channel_id.send_message(ctx, |m| {
        m.reference_message(msg);
        m.embed(|e| {
            e.color(ERROR_COLOR);
            e.description("Invaid index.");
            e
        });
        m
    }).await);
}

pub async fn url_error (ctx: &Context, msg: &Message) {
    check_msg(msg.channel_id.send_message(ctx, |m| {
        m.reference_message(msg);
        m.embed(|e| {
            e.color(ERROR_COLOR);
            e.description("Issue retrieving content from url. (Invalid URL, Geo-Blocked, Age-Restricted content.)");
            e
        });
        m
    }).await);
}

pub async fn url_missing (ctx: &Context, msg: &Message) {
    check_msg(msg.channel_id.send_message(ctx, |m| {
        m.reference_message(msg);
        m.embed(|e| {
            e.color(ERROR_COLOR);
            e.description("Must provide a query or a url");
            e
        });
        m
    }).await);
}

pub async fn search_error (ctx: &Context, msg: &Message) {
    check_msg(msg.channel_id.send_message(ctx, |m| {
        m.reference_message(msg);
        m.embed(|e| {
            e.color(ERROR_COLOR);
            e.description("Issue retrieving content from search. (Geo-Blocked, Age-Restricted content.)");
            e
        });
        m
    }).await);
}

pub async fn track_error (ctx: &Context, msg: &Message) {
    check_msg(msg.channel_id.send_message(ctx, |m| {
        m.reference_message(msg);
        m.embed(|e| {
            e.color(ERROR_COLOR);
            e.description("Couldn't get the info of the currently playing track.");
            e
        });
        m
    }).await);
}

pub async fn already_in_vc (ctx: &Context, msg: &Message) {
    check_msg(msg.channel_id.send_message(ctx, |m| {
        m.reference_message(msg);
        m.embed(|e| {
            e.color(ERROR_COLOR);
            e.description("Could not join voice channel (Already connected).");
            e
        });
        m
    }).await);
}

pub async fn cant_shuffle_0 (ctx: &Context, msg: &Message) {
    check_msg(msg.channel_id.send_message(ctx, |m| {
        m.reference_message(msg);
        m.embed(|e| {
            e.color(ERROR_COLOR);
            e.description("No songs in the queue to shuffle.");
            e
        });
        m
    }).await);
}

pub async fn cant_shuffle_1 (ctx: &Context, msg: &Message) {
    check_msg(msg.channel_id.send_message(ctx, |m| {
        m.reference_message(msg);
        m.embed(|e| {
            e.color(ERROR_COLOR);
            e.description("Cannot shuffle 1 song.");
            e
        });
        m
    }).await);
}



/// ========================
/// 
///     SETTINGS MESSAGES
/// 
/// ========================



pub async fn changed_prefix (ctx: &Context, msg: &Message, prefix: &str) {
    check_msg(msg.channel_id.send_message(ctx, |m| {
        m.reference_message(msg);
        m.embed(|e| {
            e.color(SETTINGS_COLOR);
            e.description(format!("Changed the command prefix to: {}", prefix));
            e
        });
        m
    }).await);
}

pub async fn current_prefix (ctx: &Context, msg: &Message, prefix: &str) {
    check_msg(msg.channel_id.send_message(ctx, |m| {
        m.reference_message(msg);
        m.embed(|e| {
            e.color(SETTINGS_COLOR);
            e.description(format!("Changed the command prefix to: {}", prefix));
            e
        });
        m
    }).await);
}