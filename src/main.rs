use poise::serenity_prelude as serenity;
// use poise::serenity_prelude::interaction;

use std::env;

use dotenv::dotenv;

mod commands;
use commands::*;

type Error = Box<dyn std::error::Error + Send + Sync>;

// This type alias will save us some typing, because the Context type is needed often
type Context<'a> = poise::Context<'a, Data, Error>;
type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, Error>;

// User data, which is stored and accessible in all command invocations
pub struct Data {}

async fn event_listener(
    _ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _user_data: &Data,
) -> Result<(), Error> {
    match event {
        poise::Event::Ready { data_about_bot } => {
            println!("{} is connected!", data_about_bot.user.name);
            let activity_name = env::var("CHAOS_BOT_ACTIVITY").unwrap_or_else(|_| "An idiot program".to_string());
            _ctx.set_activity(serenity::Activity::watching(activity_name)).await;
        }
        poise::Event::GuildMemberAddition { new_member } => {
            let role_id = env::var("CHAOS_BOT_JOIN_ROLE").expect("missing join role id").parse::<u64>().unwrap();
            new_member.clone().add_role(&_ctx.http, role_id).await.expect("unable to add role");
        }
        poise::Event::InteractionCreate { interaction } => {
            // TODO: Implement response for button interactions
            match interaction {
                serenity::Interaction::Ping(_) => (),
                serenity::Interaction::ApplicationCommand(_) => {
                    let i = interaction.clone();
                    let i = i.application_command().unwrap();
                    println!("[cmd] {} : {}", &i.id, &i.data.name);
                },
                serenity::Interaction::MessageComponent(_) => {
                    let i = interaction.clone();
                    let i = i.message_component().unwrap();
                    println!("[btn] {} : {}", &i.id, &i.data.custom_id);
                },
                serenity::Interaction::Autocomplete(_) => (),
                serenity::Interaction::ModalSubmit(_) => (),
            }
        }
        _ => {}
    }

    Ok(())
}

/// Registers or unregisters application commands in this guild or globally
#[poise::command(prefix_command, slash_command, ephemeral, default_member_permissions = "ADMINISTRATOR")]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("CHAOS_BOT_TOKEN").expect("missing token");

    let options = poise::FrameworkOptions {
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("$".into()),
            ..Default::default()
        },
        commands: vec![
            commands::misc::account_age(), 
            register(),
            misc::say(),
            misc::about(),
            misc::cookie(),
            misc::hype(),
            misc::poll3(),
            misc::delete(),
            misc::magic_8_ball(),
            // pet::pet(),
            admin::lock(),
            admin::archive(),
            admin::emoji(),
            admin::modal(),
            admin::hammer(),
            report::report(),
            admin::voice_ban(),
            admin::voice_unban(),
        ],
        listener: |ctx, event, framework, user_data| {
            Box::pin(event_listener(ctx, event, framework, user_data))
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .token(token)
        .intents(serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::GUILDS | serenity::GatewayIntents::MESSAGE_CONTENT | serenity::GatewayIntents::GUILD_MEMBERS)
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }))
        .options(options);

    framework.run().await.unwrap();
}
