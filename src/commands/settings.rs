use crate::{
    Database, 
    util::{
        misc::check_msg,
        embeds,
    },
};

use serenity::{
    framework::standard::{macros::command, Args, ArgError, CommandResult},
    model::prelude::*,
    prelude::*,
};



#[command]
#[only_in(guilds)]
#[required_permissions(MANAGE_GUILD)]
#[aliases(setting, options, option)]
async fn settings(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    let setting = args.single::<String>();
    
    // If the message doesn't have a guild id attached to it, return the default id
    let guild_id: u64 = match msg.guild_id {
        Some(id) => id.0,
        None => serenity::model::id::GuildId(0).0,
    };

    match setting {
        Ok(s) => match s.to_lowercase().as_str() {   
            // Prefix Setting
            "prefix" => {

                let data = ctx.data.read().await;
                let database = data.get::<Database>().expect("Expected Database in TypeMap");

                let arg1 = args.single::<String>();
                match arg1 {    
                    // If a Prefix is specififed, change the prefix, 
                    // unless the guild id is 0, in which case return the 0 (default) prefix
                    Ok(s) => { 
                        database.lookup("guild_settings", guild_id).await;
                        database.update("guild_settings", "prefix", &s, guild_id).await;
                        embeds::changed_prefix(ctx, msg, &s).await;
                        // check_msg(msg.channel_id.say(&ctx.http, format!("`PLACEHOLDER` Changed prefix to: {}", s)).await);
                    } 
                    
                    // if no prefix is specified, say the current prefix for the server.
                    Err(_) => {
                        database.lookup("guild_settings", guild_id).await;
                        let prefix = database.retrieve_str("guild_settings", "prefix", guild_id).await;
                        embeds::current_prefix(ctx, msg, &prefix).await;
                        //check_msg(msg.channel_id.say(&ctx.http, format!("`PLACEHOLDER` Guild's current prefix: {}", prefix)).await);
                    }
                }            
            },

            "dj" => {

                let data = ctx.data.read().await;
                let database = data.get::<Database>().expect("Expected Database in TypeMap");

                let arg1 = args.single::<RoleId>();
                match arg1 {
                    Ok(s) => {
                        if let Some(dj_role) = s.to_role_cached(&ctx.cache).await {
                            database.lookup("guild_settings", guild_id).await;
                            database.update("guild_settings", "dj_role_id", format!("{}", &s.0).as_str(), guild_id).await;
                            check_msg(msg.channel_id.say(&ctx.http, format!("`PLACEHOLDER` Set DJ role to: `{}, {}`", dj_role.name, dj_role.id.0)).await);
                        } else {
                            check_msg(msg.channel_id.say(&ctx.http, "`PLACEHOLDER` Argument provided was not a role.").await);
                        }
                    }

                    Err(err) => {
                        match err {
                            ArgError::Parse(_) => {
                                check_msg(msg.channel_id.say(&ctx.http, "`PLACEHOLDER` Argument provided was not a role.").await);
                            }

                            _ => {
                                database.lookup("guild_settings", guild_id).await;
                                let dj_role_i64: i64 = database.retrieve_int("guild_settings", "dj_role_id", guild_id).await;
                                if dj_role_i64 == 0 {
                                    check_msg(msg.channel_id.say(&ctx.http, "`PLACEHOLDER` This server hasn't configured a DJ role.").await);
                                } else {
                                    let dj_role_id = RoleId(dj_role_i64 as u64);
                                    if let Some(dj_role) = dj_role_id.to_role_cached(&ctx.cache).await {
                                        check_msg(msg.channel_id.say(&ctx.http, format!("`PLACEHOLDER` Configured DJ role: `{}, {}`", dj_role.name, dj_role.id.0)).await);
                                    } else {
                                        check_msg(msg.channel_id.say(&ctx.http, "`PLACEHOLDER` The Configured DJ role no longer exists!").await);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            // If a setting is specified which doesn't exist, declare as much, and print a list of available settings.
            _ => check_msg(msg.channel_id.say(&ctx.http, format!("`PLACEHOLDER` Unavailable setting specified: {}", s)).await),
        },

        // If no setting is specified, pring a list of available settings.
        Err(_) => check_msg(msg.channel_id.say(&ctx.http, "`PLACEHOLDER` No setting specified").await),
    };

    Ok(())
}