use crate::{
    commands::music::millis_to_string,
    responses::{self, Say},
    Context, Error,
};

/// Displays the queue.
#[poise::command(slash_command)]
pub async fn queue(
    context: Context<'_>,
    #[description = "The page of the queue that you would like to view."] page: Option<usize>,
) -> Result<(), Error> {
    let guild = context.guild().unwrap();
    let guild_id = guild.id;

    let lava_client = context.data().lavalink.clone();

    let Some(player_context) = lava_client.get_player_context(guild_id) else {
        responses::ErrorMessage::BotNotInVC.say(context).await?;
        return Ok(());
    };

    let currently_playing = player_context.get_player().await?.track;

    if currently_playing.is_none() {
        responses::ErrorMessage::BotNotPlaying.say(context).await?;
        return Ok(());
    }

    let currently_playing = currently_playing.unwrap();
    let queue = player_context.get_queue().await?;
    let queue_length = queue.len();
    let page = if page.is_some() { page.unwrap() } else { 1 };

    // Calculate how many pages the queue should have
    let mut pages = (queue_length + 9) / 10;
    if pages == 0 {
        pages = 1
    }

    // Test if the page provided is valid
    if pages < page || page < 1 {
        responses::error(context, "Invaild page specified.").await?;
        return Ok(());
    }

    let mut total_length: u64 = 0;
    let mut queue_string = String::default();

    let title = currently_playing.info.title;
    let duration = currently_playing.info.length;

    total_length += duration;

    match &currently_playing.info.uri {
        Some(uri) => queue_string.push_str(
            format!(
                "**Now Playing**: [{}]({}) `{}`\n\n",
                title,
                uri,
                millis_to_string(currently_playing.info.length),
            )
            .as_str(),
        ),
        None => queue_string.push_str(
            format!(
                "**Now Playing**: {} `{}`\n\n",
                title,
                millis_to_string(currently_playing.info.length),
            )
            .as_str(),
        ),
    }

    if queue_length >= 1 {
        queue_string.push_str("**Up Next**\n");

        // We don't want to get more than 10 songs
        let queue_length_adjusted: usize;
        if queue_length > page * 10 {
            queue_length_adjusted = 10
        } else {
            queue_length_adjusted = queue_length
        }

        for (count, wrapped_track) in queue
            .range((page - 1) * 10..queue_length_adjusted)
            .enumerate()
        {
            let track = &wrapped_track.track;
            let title = &track.info.title;
            let duration = track.info.length;
            let position = (page - 1) * 10 + count + 1;
            total_length += duration;

            match &track.info.uri {
                Some(uri) => queue_string.push_str(
                    format!(
                        "**{}.** [{}]({}) `{}`\n",
                        position,
                        title,
                        uri,
                        millis_to_string(duration)
                    )
                    .as_str(),
                ),
                None => queue_string.push_str(
                    format!(
                        "**{}.** {} `{}`\n",
                        position,
                        title,
                        millis_to_string(duration)
                    )
                    .as_str(),
                ),
            }

            // If the string exceeds the maximum allowed characters
            if queue_string.len() == 2000 {
                break;
            }
        }
    }

    let queue_footer = format!(
        "Page {}/{} | {} song(s) in queue | {} total duration",
        page,
        pages,
        queue_length,
        millis_to_string(total_length),
    );

    context
        .send(|message| {
            message.embed(|embed| {
                embed
                    .description(queue_string)
                    .footer(|footer| footer.text(queue_footer))
                    .color(responses::Color::Default.to_color())
            })
        })
        .await?;

    Ok(())
}
