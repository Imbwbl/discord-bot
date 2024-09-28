use crate::{Context, Error};
use poise::{serenity_prelude as serenity, CreateReply};
use serenity::all::GetMessages;
use std::time::Duration;

#[poise::command(slash_command, prefix_command)]
pub async fn clear(
    ctx: Context<'_>,
    #[description = "number of messages to delete"] number: u8,
) -> Result<(), Error> {
    let channel_id = ctx.channel_id();

    let messages = channel_id
        .messages(ctx, GetMessages::default().limit(number))
        .await?;

    channel_id
        .delete_messages(ctx, messages.iter().map(|msg| msg.id))
        .await
        .expect("failed to delete messages from channel");

    let message = ctx
        .send(
            CreateReply::default()
                .content(format!("Deleted {} messages.", messages.len()))
                .ephemeral(true),
        )
        .await?;
    tokio::time::sleep(Duration::from_secs(5)).await;
    message.delete(ctx).await?;

    Ok(())
}
