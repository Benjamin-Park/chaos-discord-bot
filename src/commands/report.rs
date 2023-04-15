use crate::{ApplicationContext, Error};
use poise::{
    serenity_prelude::{self as serenity, ChannelId, /*Embed*/},
    Modal,
};

static REPORT_CHANNELID: u64 = 998541060202569779;

#[derive(Debug, Modal)]
#[name = "Report User"] // Struct name by default
struct ReportModal {
    #[name = "Reason"] // Field name by default
    #[placeholder = "Your first input goes here"]
    // No placeholder by default
    // #[min_length = 5] // No length restriction by default (so, 1-4000 chars)
    #[max_length = 500]
    reason: String,
    #[name = "Additonal Information"]
    #[paragraph] // Switches from single-line input to multiline text box
    info: Option<String>, // Option means optional input
}

/// Report a user to the Server Moderation Team
#[poise::command(slash_command)]
pub async fn report(
    ctx: ApplicationContext<'_>,
    #[description = "User to report"] user: serenity::User,
) -> Result<(), Error> {
    let data = ReportModal::execute(ctx).await?;
    let reporter = ctx.interaction.user();
    println!("Got data: {:?}", data);

    ChannelId(REPORT_CHANNELID)
        .send_message(&ctx.discord.http, |m| {
            m.embed(|e| {
                e.title("Report")
                    .thumbnail(user.avatar_url().unwrap())
                    .colour(0xD44B4B)
                    .author(|a| a.name(&user.name).icon_url(user.avatar_url().unwrap()))
                    .field("Reported User", user, true)
                    .field("Reported By", reporter, true)
                    .field("", "", false)
                    .field(format!("Reason: {}", data.reason), data.info.unwrap_or_default(), false)
                    // .timestamp(chrono::DateTime)
            })
            // .components(|c| c.create_action_row(|r| 
                // r.create_button(|b| b.label("Test").custom_id("button_test"))))
        })
        .await?;

    // reporter.direct_message(&ctx.discord.http, |m| {m.content("Thank you for your report")}).await?;
    // ctx.interaction.channel_id().say(&ctx.discord.http, format!("Report Recieved from {}", reporter)).await?;
    Ok(())
}
