use std::vec;

// use crate::{Context, Data, Error};
use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity, ReactionType};
use rand;
use rand::seq::SliceRandom;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
pub async fn account_age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

// FIXME: Update required permissions or remove command
/// Send a message containing the supplied text
#[poise::command(slash_command, prefix_command, aliases("echo"), default_member_permissions = "ADMINISTRATOR")]
pub async fn say(
    ctx: Context<'_>,
    #[description = "Text to send"] msg: String,
) -> Result<(), Error> {
    ctx.say(msg).await?;

    return Ok(());
}

/// Display information about the bot
#[poise::command(prefix_command, slash_command)]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(|m| {m.embed(|e| {
        e.title(format!("ChaosBot v{}", env!("CARGO_PKG_VERSION"))).description("[WIP]")
        .thumbnail("https://cdn.discordapp.com/avatars/998355924777435268/b68edcec0230d8c9e31db34408e94de8.png")
        .field(
            "About",
            "Experimental bot developed for the Chaos Crew offical Discord server.",
            true,
        )
        .field(
            "Features",
            "
            - Stuff
            - And things
            - Mostly doesn't break
            - (Too much)
            ",
            true,
        )
        .footer(|a| {
            a.text("Created by Fulcrum")
            .icon_url("https://cdn.discordapp.com/avatars/783996731926773794/1f1736d58ec838fb686af11f823740b3.png")
        })
        .colour(0x5539cc)
        })
    }).await?;

    return Ok(());
}

/// Be annoying to your (ex)friends
#[poise::command(slash_command, ephemeral)]
pub async fn poke(
    ctx: Context<'_>,
    #[description = "Users to mention"] users: Vec<String>,
) -> Result<(), Error> {
    for user in users {
        ctx.channel_id()
            .send_message(&ctx.discord().http, |m| m.content(user))
            .await?;
    }

    ctx.say("Complete!").await?;

    return Ok(());
}

// Magic 8 Ball
/// It's like any other 8ball command on discord. Annoying, useless and unreasonably popular.
#[poise::command(slash_command, rename = "8ball")]
pub async fn magic_8_ball(
    ctx: Context<'_>,
    #[description = "Question with Yes/No answer"] question: String,
) -> Result<(), Error> {
    let responses: Vec<&str> = vec![
        "All signs point to yes...",
        "Yes!",
        "My sources say nope.",
        "You may rely on it.",
        "Concentrate and ask again...",
        "Outlook not so good...",
        "It is decidedly so!",
        "Better not tell you.",
        "Very doubtful.",
        "Yes - Definitely!",
        "It is certain!",
        "Most likely.",
        "Ask again later.",
        "No!",
        "Outlook good.",
        "Don't count on it.",
    ];

    let response = responses
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string();
    ctx.say(format!("> {} \n{}", question, response)).await?;

    return Ok(());
}

/// Animated Hype Emote
#[poise::command(slash_command, prefix_command)]
pub async fn hype(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("https://cdn.discordapp.com/emojis/1032184464744403004.gif?size=48&quality=lossless").await?;

    return Ok(());
}

/// Give out a cookie
#[poise::command(slash_command, prefix_command)]
pub async fn cookie(
    ctx: Context<'_>,
    #[description = "username"] recipient: Option<serenity::User>,
) -> Result<(), Error> {
    let author = &ctx.author().name;
    let recipient = match recipient {
        Some(_) => recipient.unwrap().name,
        _ => "out".to_string(),
    };
    ctx.say(format!("{} has given {} a cookie! :cookie:", author, recipient)).await?;

    return Ok(());
}

/// delete a message by id
#[poise::command(slash_command, prefix_command, ephemeral, default_member_permissions = "ADMINISTRATOR")]
pub async fn delete(
    ctx: Context<'_>,
    #[description = "id"] message_id: String,
) -> Result<(), Error> {
    let message_id = message_id.parse::<u64>().unwrap();
    ctx.channel_id().delete_message(&ctx.discord().http, message_id).await?;
    ctx.say("success").await?;

    return Ok(());
}

/// Create a poll with 3 options
#[poise::command(slash_command, prefix_command)]
pub async fn poll3(
    ctx: Context<'_>,
    #[description = "channel to send poll"] channel: serenity::ChannelId,
     #[description = "to vote on..."] poll: String,
    option1: String,
    option2: String,
    option3: String,
) -> Result<(), Error> {
    let msg = channel.say(&ctx.discord().http, format!("> **React to vote on {}:** \n :one: = {}\n :two: = {}\n :three: = {}\n", poll, option1, option2, option3)).await?;
    
    msg.react(&ctx.discord().http, ReactionType::Unicode("1️⃣".to_string())).await?;
    msg.react(&ctx.discord().http, ReactionType::Unicode("2️⃣".to_string())).await?;
    msg.react(&ctx.discord().http, ReactionType::Unicode("3️⃣".to_string())).await?;
    
    ctx.say(format!("Poll created in <#{}>", channel)).await?;

    return Ok(());
}
