/*
 * Copyright (c) 2022 tooboredtocode
 * All Rights Reserved
 */

use tracing::info;
#[cfg(debug_assertions)]
use tracing::warn;
use twilight_model::id::Id;

use crate::context::Ctx;
use crate::util::error::Expectable;
use crate::ShareResult;

pub mod pin;

pub async fn sync_commands(ctx: &Ctx) -> ShareResult<()> {
    info!("Syncing commands");
    sync(ctx).await?;
    info!("Successfully synced all commands");

    Ok(())
}

#[cfg(debug_assertions)]
async fn sync(ctx: &Ctx) -> ShareResult<()> {
    if ctx.cfg.debug_server.len() == 0 {
        warn!("No Debug Servers were configured")
    }

    for debug_server in &ctx.cfg.debug_server {
        ctx.interaction_client()
            .set_guild_commands(Id::new(*debug_server), &[pin::command()])
            .await
            .expect_with("Failed to Synchronize Commands")?;
    }

    Ok(())
}

#[cfg(not(debug_assertions))]
async fn sync(ctx: &Ctx) -> ShareResult<()> {
    ctx.interaction_client()
        .set_global_commands(&[pin::command()])
        .exec()
        .await
        .expect_with("Failed to Synchronize Commands")?;

    Ok(())
}
