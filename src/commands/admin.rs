use crate::{Context, Error, ApplicationContext};
use poise::{serenity_prelude::{self as serenity, SerenityError, ChannelId, /*UserId,*/ ChannelType}, Modal};
// use poise::serenity_prelude::{self as serenity, Emoji, Guild, EmojiId};
use ::serenity::model::channel::{PermissionOverwrite, PermissionOverwriteType};
use ::serenity::model::permissions::Permissions;

/// Archives a channel
#[poise::command(slash_command, prefix_command, default_member_permissions = "MANAGE_CHANNELS")]
pub async fn archive(
    ctx: Context<'_>,
    #[description = "Channel to archive"] mut channel: serenity::GuildChannel,
) -> Result<(), Error> {
    // manual parse channel from id
    // let channel_id = ChannelId::from(channel.replace(['#', '<', '>'], "").parse::<u64>()?);
    
    // Manually set channel permissions
    // let perms = vec![PermissionOverwrite {
    //     allow: Permissions::VIEW_CHANNEL,
    //     deny: Permissions::SEND_MESSAGES,
    //     kind: PermissionOverwriteType::Role(RoleId(784268832814268446)),
    // }];
    // let archive = ChannelId(1068083738195546123); // Testing Server
    let archive = ChannelId(1016611522644017213); // Production
    // Requires bot to have permission to view category channels
    let category = archive.to_channel(&ctx.discord().http).await;
    // TODO: cleanupError handling when missing access
    if let Err(e) = &category {
        ctx.say(format!("Error: {}", e)).await?;
        return Ok(());
    }
    
    match channel.edit(&ctx.discord().http, |c| c.category(archive).permissions(category.unwrap().category().unwrap().permission_overwrites)).await {
        Ok(_) => ctx.say(format!("Archived channel {}", channel)).await?,
        Err(e) => ctx.say(format!("Error: {}", e)).await?,
    };

    Ok(())
}

/// Locks an emoji to a role
#[poise::command(slash_command, prefix_command, default_member_permissions = "MANAGE_EMOJIS_AND_STICKERS")]
pub async fn lock(
    ctx: Context<'_>,
    #[description = "Emoji to lock"] emoji: String,
    #[description = "Role allowed to use emoji"] role: Option<serenity::Role>,
) -> Result<(), Error> {
    let id = serenity::utils::parse_emoji(&emoji).unwrap();

    let mut emoji = serenity::Guild::emoji(&ctx.guild().unwrap(), &ctx.discord().http, id.id)
        .await
        .unwrap();

    let role = role.unwrap();

    emoji.roles.push(role.id);

    emoji.delete(ctx.discord()).await.unwrap();

    // ctx.say(format!("{} locked to {:?}", emoji.name, emoji.roles)).await?;

    Ok(())
}

/// Commands for managing your server emojis
#[poise::command(slash_command, subcommands("info", "delete", "lock"))]
pub async fn emoji(_ctx: Context<'_>) -> Result<(), Error> {
    return Ok(());
}

#[poise::command(slash_command, prefix_command)]    
pub async fn info(ctx: Context<'_>, emoji: String) -> Result<(), Error> {
    ctx.say("Info").await?;
    ctx.say(format!("Emoji: {}", emoji)).await?;

    return Ok(());
}

#[poise::command(slash_command, prefix_command, default_member_permissions = "MANAGE_EMOJIS_AND_STICKERS")]
pub async fn delete(ctx: Context<'_>, emoji: String) -> Result<(), Error> {
    let id = serenity::utils::parse_emoji(&emoji);

    let emoji = serenity::Guild::emoji(&ctx.guild().unwrap(), &ctx.discord().http, id.unwrap().id)
        .await
        .unwrap();

    match emoji.delete(&ctx.discord()).await {
        Ok(()) => {
            ctx.say("Emoji deleted").await?;
        },
        Err(SerenityError::Http(_)) => {
            ctx.say("Emoji not found").await?;
        },
        Err(SerenityError::Model(_)) => {
            ctx.say("Invalid Permissions").await?;
        },
        Err(why) => {
            println!("Unexpected error: {:?}", why);
        },
    }

    return Ok(());
}

// Modal and Button testing
#[derive(Debug, Modal)]
#[name = "Modal title"] // Struct name by default
struct MyModal {
    #[name = "First input label"] // Field name by default
    #[placeholder = "Your first input goes here"] // No placeholder by default
    #[min_length = 5] // No length restriction by default (so, 1-4000 chars)
    #[max_length = 500]
    first_input: String,
    #[name = "Second input label"]
    #[paragraph] // Switches from single-line input to multiline text box
    second_input: Option<String>, // Option means optional input
}

#[poise::command(slash_command)]
pub async fn modal(ctx: ApplicationContext<'_>) -> Result<(), Error> {
    let data = MyModal::execute(ctx).await?;
    println!("Got data: {:?}", data);

    Ok(())
}

/// A mods daily dopamine hit
#[poise::command(slash_command, default_member_permissions = "VIEW_AUDIT_LOG")]
pub async fn hammer(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("https://tenor.com/view/bongocat-banhammer-ban-hammer-bongo-gif-18219363").await?;

    Ok(())
}

/// Restrict a user from connecting to voice channels
#[poise::command(slash_command, prefix_command, default_member_permissions = "MUTE_MEMBERS")]
pub async fn voice_ban(
    ctx: Context<'_>,
    #[description = "Selected user"] user: serenity::User,
) -> Result<(), Error> {
    // let general_category = ChannelId(888657317996929055);
    let ban_perms = PermissionOverwrite {
        allow: Permissions::empty(),
        deny: Permissions::CONNECT,
        kind: PermissionOverwriteType::Member(user.id),
    };
    
    let guild_channels = ctx.guild().unwrap().channels(&ctx.discord().http).await.unwrap();
    for (_, channel) in guild_channels {
        if channel.kind == ChannelType::Voice && channel.kind != ChannelType::Private {
            println!("{}", channel.name());
            channel.create_permission(&ctx.discord().http, &ban_perms).await?;
        }
    }
    // ChannelId(784268832814268449).create_permission(&ctx.discord().http, &ban_perms).await?; // DEBUG: Remove
    
    ctx.say(format!("{} Restricted from voice channels", user)).await?;
    Ok(())
}

/// Remove a users restriction on connecting to voice channels
#[poise::command(slash_command, prefix_command, default_member_permissions = "MUTE_MEMBERS")]
pub async fn voice_unban(
    ctx: Context<'_>,
    #[description = "Selected user"] user: serenity::User,
) -> Result<(), Error> {    
    // ChannelId(784268832814268449).delete_permission(&ctx.discord().http, PermissionOverwriteType::Member(user.id)).await?;
    
    let guild_channels = ctx.guild().unwrap().channels(&ctx.discord().http).await.unwrap();
    for (_channel_id, channel) in guild_channels {
        if channel.kind == ChannelType::Voice && channel.kind != ChannelType::Private {
            println!("{}", channel.name);
            channel.delete_permission(&ctx.discord().http, PermissionOverwriteType::Member(user.id)).await?;
        }
    }
    
    ctx.say(format!("Restored voice channel acess for {}", user)).await?;
    Ok(())
}
