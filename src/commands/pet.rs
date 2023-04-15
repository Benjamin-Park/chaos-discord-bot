#![allow(unused)] // FIXME: 
#![allow(dead_code)] // FIXME: 

use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity, CreateEmbed, Embed};

// use rusqlite::{self, Statement};

enum PetMood {
    Neutral,
    Happy,
    Grumpy,
    Sad,
}

struct Pet {
    name: String,
    owner_id: i64,
    mood: PetMood,
    last_interaction: i64
}

// fn get_pet() -> rusqlite::Result<()> {
//     let connection = rusqlite::Connection::open("userData.db".to_string()).expect("error connecting to database");
    
//     let mut statement = connection.prepare("SELECT * FROM users")?;
    
//     print!(statement.query_map().to_string());
    
//     return Ok(());    
// }

/// Pet Display
// fn pet_display(ctx: Context<'_>) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
//     let embed = CreateEmbed::default()
//         .author(|a| {
//             a.name("Pet")
//                 .icon_url("https://rustacean.net/assets/cuddlyferris.png")
//         })
//         .title(format!("{}'s Pet", ctx.author().name))
//         .description(format!("{} is {}", "Pet Name", "Pet Type"))
//         .image("https://rustacean.net/assets/cuddlyferris.png")
//         .clone();

//     return embed;
// }

async fn pet_embed(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(|m| {
        m.embed(|e| {
            e.title(format!("{}'s Pet", ctx.author().name))
                .description(format!("{} is {}", "Pet Name", "Pet Type"))
                .image("https://rustacean.net/assets/cuddlyferris.png")
        })
    })
    .await?;

    Ok(())
}

/// Commands for managing your pet
#[poise::command(slash_command, prefix_command, subcommands("info", "test"))]
pub async fn pet(_ctx: Context<'_>) -> Result<(), Error> {
    return Ok(());
}

/// Displays your pets' current info
#[poise::command(slash_command, prefix_command)]
async fn info(ctx: Context<'_>) -> Result<(), Error> {
    pet_embed(ctx).await?;

    let reply = ctx
        .send(|m| {
            m.content("Pet Info").components(|c| {
                c.create_action_row(|r| {
                    r.create_button(|b| {
                        b.custom_id("test")
                            .label("Very useful button")
                            .style(serenity::ButtonStyle::Primary)
                    })
                })
            })
        })
        .await?;

    return Ok(());
}

/// Displays your pets' current info
#[poise::command(slash_command, prefix_command)]
async fn test(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("subcommand2").await?;

    return Ok(());
}
