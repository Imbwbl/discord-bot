use crate::{Context, Data, Error};
use poise::serenity_prelude::ChannelId;
use poise::{serenity_prelude as serenity, CreateReply};
use serenity::all::{Channel, ChannelType, CreateMessage, Mentionable, PermissionOverwrite, PermissionOverwriteType, Permissions, RoleId};
use serenity::builder::CreateChannel;

use ChannelType::Voice;
use serenity::all::FullEvent::CategoryCreate;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }
        serenity::FullEvent::GuildMemberAddition { new_member } => {
            let channel = ChannelId::new(1289591846703595613);
            channel
                .send_message(
                    &ctx,
                    CreateMessage::default().content(format!(
                        "Welcome to the server, {}! 🎉",
                        new_member.mention()
                    )),
                )
                .await?;
        }
        serenity::FullEvent::VoiceStateUpdate { old, new } => {

            // User joins a voice channel
            let category_id = ChannelId::new(1289636827304820868);
            let trigger_channel_id = ChannelId::new(1289587528432615527);

            if let Some(new_channel_id) = new.channel_id {
                let guild_id = new.guild_id;
                if new_channel_id == trigger_channel_id {
                    let user_id = new.user_id;
                    let user = user_id.to_user(ctx).await?;
                    let user_name = user.name;
                    let permissions = vec![
                        PermissionOverwrite {
                            allow: Permissions::all(),
                            deny: Permissions::empty(),
                            kind: PermissionOverwriteType::Member(user_id),
                        },
                    ];
                    let channel = guild_id.unwrap()
                        .create_channel(&ctx.http, CreateChannel::new(format!("Voice {}", user_name))
                                .kind(ChannelType::Voice)
                                .category(category_id)
                                .permissions(permissions)
                        )
                        .await?;
                    guild_id.unwrap().move_member(&ctx.http, user_id, channel.id).await?;
                }
            }

            // User leaves a voice channel
            if let Some(old_voice_state) = old {
                if let Some(left_channel_id) = old_voice_state.channel_id {
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    let channel = left_channel_id.to_channel(ctx).await?;
                    if let serenity::Channel::Guild(channel) = channel {
                        if channel.kind == ChannelType::Voice || channel.kind == ChannelType::Stage {
                            if channel.parent_id == Some(ChannelId::new(1289636827304820868)) {
                                let members = channel.members(ctx);
                                if members?.is_empty() {
                                    channel.id.delete(ctx).await?;
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}
