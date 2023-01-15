/*
 * Copyright (c) 2022 tooboredtocode
 * All Rights Reserved
 */

use tracing::warn;
use twilight_model::application::interaction::application_command::CommandData;
use twilight_model::application::interaction::Interaction;
use twilight_model::channel::message::MessageFlags;
use twilight_model::channel::Message;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::context::Ctx;
use crate::util::error::Expectable;
use crate::util::EmptyResult;

pub fn get_message(data: &CommandData) -> EmptyResult<&Message> {
    let resolved = match &data.resolved {
        None => {
            warn!("Received Message Application Command Interaction without resolved data");
            return Err(());
        }
        Some(r) => r,
    };

    match resolved.messages.iter().next() {
        None => {
            warn!("Received Message Application Command Interaction without message");
            Err(())
        }
        Some((_, msg)) => Ok(msg),
    }
}

pub async fn defer(inter: &Interaction, context: &Ctx) -> EmptyResult<()> {
    if let Err(_) = context
        .interaction_client()
        .create_response(
            inter.id,
            inter.token.as_str(),
            &InteractionResponse {
                kind: InteractionResponseType::DeferredChannelMessageWithSource,
                data: None,
            },
        )
        .await
    {
        warn!("Failed to defer Response, aborting handler");
        return Err(());
    }

    Ok(())
}

pub async fn respond_with(inter: &Interaction, context: &Ctx, msg: &str) -> EmptyResult<()> {
    context
        .interaction_client()
        .create_response(
            inter.id,
            inter.token.as_str(),
            &InteractionResponse {
                kind: InteractionResponseType::ChannelMessageWithSource,
                data: InteractionResponseDataBuilder::new()
                    .flags(MessageFlags::EPHEMERAL)
                    .content(msg)
                    .build()
                    .into(),
            },
        )
        .await
        .warn_with("Failed to respond to the Interaction")
        .map(|_| ())
        .ok_or(())
}
