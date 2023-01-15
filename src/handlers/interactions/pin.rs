/*
 * Copyright (c) 2022 tooboredtocode
 * All Rights Reserved
 */

use tracing::{debug, info, instrument, warn};
use twilight_http::{error::ErrorType as HttpErrorType, request::AuditLogReason};
use twilight_model::application::interaction::application_command::CommandData;
use twilight_model::application::interaction::Interaction;
use twilight_model::channel::Message;
use twilight_model::id::marker::GuildMarker;
use twilight_model::id::Id;
use twilight_util::builder::embed::EmbedBuilder;

use crate::context::Ctx;
use crate::handlers::interactions::messages::{audit_log_reason, error, missing_permissions};
use crate::util::discord_locales::DiscordLocale;
use crate::util::error::Expectable;
use crate::util::interaction::{defer, get_message, respond_with};
use crate::util::EmptyResult;

pub async fn handle(inter: &Interaction, data: &CommandData, context: Ctx) {
    // use an inner function to make splitting the code easier
    let _ = handle_inner(inter, data, context).await;
}

#[instrument(name = "pin_command_handler", level = "debug", skip_all)]
async fn handle_inner(inter: &Interaction, data: &CommandData, context: Ctx) -> EmptyResult<()> {
    debug!("Received Pin Command Interaction");

    defer(inter, &context).await?;

    let msg = get_message(data)?;
    let user = inter.member.as_ref().and_then(|m| m.user.as_ref());

    let locale = (&inter.locale).into();

    let res = if msg.pinned {
        context
            .discord_client
            .delete_pin(msg.channel_id, msg.id)
            .reason(&audit_log_reason(locale, user))
            .warn_with("Static message somehow failed validation")
            .ok_or(())?
            .await
    } else {
        context
            .discord_client
            .create_pin(msg.channel_id, msg.id)
            .reason(&audit_log_reason(locale, user))
            .warn_with("Static message somehow failed validation")
            .ok_or(())?
            .await
    };

    if let Err(e) = res {
        match e.kind() {
            HttpErrorType::Response { status, .. } if status.get() == 403 => {
                info!("Did not have permission to pin message, informing user");
                respond_with(inter, &context, missing_permissions(locale, msg.pinned)).await?;
            }
            _ => {
                warn!(
                    "Failed to {}pin message, returning the error",
                    if msg.pinned { "un" } else { "" }
                );
                respond_with(inter, &context, error(locale)).await?;
            }
        }

        return Err(());
    }

    let user_mention = match user {
        Some(user) => format!("<@{}>", user.id),
        None => "Unknown User".to_string(),
    };

    let message_link = if let Some(id) = inter.guild_id {
        get_message_link(msg, id)
    } else {
        "".to_string()
    };

    let description = match (locale, msg.pinned) {
        (DiscordLocale::GERMAN, true) => format!(
            "[Nachricht]({}) für {} losgelöst",
            message_link, user_mention
        ),
        (DiscordLocale::GERMAN, false) => format!(
            "[Nachricht]({}) für {} angepinnt",
            message_link, user_mention
        ),
        (_, true) => format!("Unpinned [message]({}) for {}", message_link, user_mention),
        (_, false) => format!("Pinned [message]({}) for {}", message_link, user_mention),
    };

    let embed = EmbedBuilder::new().description(description).build();

    let r = context
        .interaction_client()
        .create_followup(inter.token.as_str())
        .embeds(&[embed])
        .expect("This should never fail, as we are using a static number of embeds")
        .await
        .warn_with("Failed to send the response to the user");

    if r.is_some() {
        debug!("Successfully sent Response");
    }

    Ok(())
}

fn get_message_link(message: &Message, guild_id: Id<GuildMarker>) -> String {
    format!(
        "https://discord.com/channels/{}/{}/{}",
        guild_id, message.channel_id, message.id
    )
}
