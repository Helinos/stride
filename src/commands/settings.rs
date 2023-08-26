use core::fmt;
use std::vec;

use crate::{Context, Error, database::{INTEGER, BOOL, ColumnType, ValidValue}};
use poise::serenity_prelude as serenity;

const GUILD_ID: &str = "guild_id";
const DJ_ID: &str = "dj_id";
const DJ_ONLY: &str = "dj_only";
const ANNOUNCE_SONGS: &str = "announce_songs";
const EVERYONE_DJ: &str = "everyone_dj";
const TABLE_NAME: &str = "guild_settings";
const TABLE_COLUMNS: [&str; 5] = [GUILD_ID, DJ_ID, DJ_ONLY, ANNOUNCE_SONGS, EVERYONE_DJ];
const TABLE_TYPES: [ColumnType; 5] = [INTEGER, INTEGER, BOOL, BOOL, BOOL];
const DEFAULT_VALUES: [&str; 5] = ["0", "0", "0", "0", "0"];

async fn update_value<T: ValidValue + fmt::Display>(context: Context<'_>, key: &str, value: &T) {
    let database = &context.data().database;
    let guild_id = context.guild_id().unwrap();
    database.update_value(TABLE_NAME, TABLE_COLUMNS.to_vec(), TABLE_TYPES.to_vec(), DEFAULT_VALUES.to_vec(), key, value, GUILD_ID, &guild_id).await;
}

// Discord doesn't permit invoking the root command of a slash command if it has subcommands, so the root command goes unused.
#[poise::command(
    slash_command,
    subcommands(
        "show",
        "dj_role",
        "dj_only",
        "everyone_dj",
        "announce_songs",
    ),
    required_permissions = "MANAGE_GUILD"
)]
pub async fn settings(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Show the bot's current settings.
#[poise::command(slash_command)]
async fn show(context: Context<'_>) -> Result<(), Error> {
    context.say("TODO: Show the settings screen").await?;
    Ok(())
}

/// Set the DJ role
#[poise::command(slash_command, ephemeral)]
async fn dj_role(
    context: Context<'_>,
    #[description = "Selected role"] role: serenity::Role,
) -> Result<(), Error> {
    update_value(context, DJ_ID, &role.id).await;
    Ok(())
}

/// Put the bot into DJ only mode
#[poise::command(slash_command)]
async fn dj_only(context: Context<'_>, boolean: bool) -> Result<(), Error> {
    update_value(context, DJ_ONLY, &boolean).await;
    Ok(())
}

/// Give every user in the server DJ permissions with the DJ role
#[poise::command(slash_command)]
async fn everyone_dj(context: Context<'_>, boolean: bool) -> Result<(), Error> {
    update_value(context, EVERYONE_DJ, &boolean).await;
    Ok(())
}

/// Set whether the bot should send a now playing message for songs as they come up in the queue
#[poise::command(slash_command)]
async fn announce_songs(context: Context<'_>, boolean: bool) -> Result<(), Error> {
    update_value(context, ANNOUNCE_SONGS, &boolean).await;
    Ok(())
}
