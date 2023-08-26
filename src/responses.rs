use poise::serenity_prelude::async_trait;
use songbird::error::JoinError;

use crate::{Context, Error};

pub enum Color {
    Default,
    Error,
    Setting,
}

impl Color {
    pub fn to_color(&self) -> poise::serenity_prelude::Color {
        use Color::*;
        match self {
            Default => poise::serenity_prelude::Color::from_rgb(149, 165, 166),
            Error => poise::serenity_prelude::Color::from_rgb(231, 76, 60),
            Setting => poise::serenity_prelude::Color::from_rgb(13, 71, 161),
        }
    }
}

#[async_trait]
pub trait Say {
    async fn say(&self, context: Context<'_>) -> Result<(), Error>;
}

pub async fn default(context: Context<'_>, description: impl Into<String>) -> Result<(), Error> {
    generic(context, Color::Default, description).await?;
    Ok(())
}

pub async fn error(context: Context<'_>, description: impl Into<String>) -> Result<(), Error> {
    generic(context, Color::Error, description).await?;
    Ok(())
}

pub async fn setting(context: Context<'_>, description: impl Into<String>) -> Result<(), Error> {
    generic(context, Color::Setting, description).await?;
    Ok(())
}

async fn generic(
    context: Context<'_>,
    color: Color,
    description: impl Into<String>,
) -> Result<(), Error> {
    context
        .send(|message| {
            message.embed(|embed| {
                embed
                    .color(color.to_color())
                    .description(description.into())
            })
        })
        .await?;

    Ok(())
}

pub enum DefaultMessage {
    Skipped,
    SkippedTo(usize),
}

#[async_trait]
impl Say for DefaultMessage {
    async fn say(&self, context: Context<'_>) -> Result<(), Error> {
        use DefaultMessage::*;
        match self {
            Skipped => default(context, "Skipped track.").await?,
            SkippedTo(position) => default(context, format!("Skipped to track {}.", position)).await?,
        }

        Ok(())
    }
}

pub enum ErrorMessage {
    UserMissingFromVC,
    BotCannotJoinVC(JoinError),
    BotNotInVC,
    BotNotPlaying,
    InvalidSkip,
    InvalidMove,
}

#[async_trait]
impl Say for ErrorMessage {
    async fn say(&self, context: Context<'_>) -> Result<(), Error> {
        use ErrorMessage::*;
        match self {
            UserMissingFromVC => error(context, "You need to be in a voice channel to use this command.").await?,
            BotCannotJoinVC(why) => error(context, format!("Could not join the voice channel. {}", why)).await?,
            BotNotInVC => error(context, "You need to be in a voice channel to use this command.").await?,
            BotNotPlaying => error(context, "There are no tracks currently playing.").await?,
            InvalidSkip => error(context, "Tried to skip to an invaild position in the queue.").await?,
            InvalidMove => error(context, "Tried to move a track to/from an invaid position.").await?,
        }

        Ok(())
    }
}
