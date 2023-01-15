/*
 * Copyright (c) 2022 tooboredtocode
 * All Rights Reserved
 */

use std::sync::Arc;

use tracing::info;
use twilight_gateway::Cluster;
use twilight_http::Client as TwilightClient;
use twilight_model::id::marker::ApplicationMarker;
use twilight_model::id::Id;

use crate::context::metrics::Metrics;
use crate::context::state::State;
use crate::util::StateUpdater;
use crate::{Config, EventPoller, ShareResult};

mod discord_client;
mod discord_cluster;
pub mod metrics;
pub mod state;

#[derive(Debug)]
pub struct Context {
    pub discord_client: TwilightClient,
    pub discord_cluster: Cluster,
    bot_id: Id<ApplicationMarker>,

    pub cfg: SavedConfig,

    pub metrics: Metrics,
    // TODO: add database for command invocation metrics
    state: State,
}

#[derive(Debug)]
pub struct SavedConfig {
    pub debug_server: Vec<u64>,
}

pub type Ctx = Arc<Context>;

impl Context {
    pub async fn new(config: &Config, snd: StateUpdater) -> ShareResult<(EventPoller, Arc<Self>)> {
        info!("Creating Cluster");

        let (discord_client, bot_id) = Self::discord_client_from_config(&config).await?;
        let (discord_cluster, events) = Self::cluster_from_config(&config).await?;

        let metrics = Metrics::new(0);

        let ctx: Arc<Self> = Context {
            discord_client,
            discord_cluster,
            bot_id,
            cfg: SavedConfig {
                debug_server: config.discord.debug_server.clone(),
            },
            metrics,
            state: State::new(snd),
        }
        .into();

        ctx.start_state_listener();
        let events_poller = EventPoller::new(events, ctx.create_state_listener());

        Ok((events_poller, ctx))
    }
}
