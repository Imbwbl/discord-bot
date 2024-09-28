use crate::{Context, Error};
use poise::CreateReply;
use std::time::Instant;

#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start = Instant::now();

    let message = ctx
        .send(CreateReply::default().content("Pong").ephemeral(true))
        .await?;

    message
        .edit(
            ctx,
            CreateReply::default().content(format!("Pong in {}ms", start.elapsed().as_millis())),
        )
        .await?;
    Ok(())
}
