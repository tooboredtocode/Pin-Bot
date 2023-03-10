/*
 * Copyright (c) 2022 tooboredtocode
 * All Rights Reserved
 */

pub const NAME: &str = "PinBot";
pub const NAME_SHORT: &str = "Pin";

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const RUST_VERSION: &str = env!("BUILD_RUSTC_VERSION");
pub const GIT_BRANCH: &str = env!("BUILD_GIT_BRANCH");
pub const GIT_REVISION: &str = env!("BUILD_GIT_REVISION");

pub mod config_consts {
    pub const YAML_FILE_PATH: &str = "config.yaml";
    pub const JSON_FILE_PATH: &str = "config.json";

    pub const ENV_PREFIX: &str = "CONFIG.";
}

pub mod cluster_consts {
    use twilight_model::gateway::payload::outgoing::update_presence::UpdatePresencePayload;
    use twilight_model::gateway::presence::{ActivityType, MinimalActivity, Status};
    use twilight_model::gateway::Intents;

    pub const GATEWAY_INTENTS: Intents = Intents::GUILDS;

    pub fn presence() -> UpdatePresencePayload {
        UpdatePresencePayload {
            activities: vec![MinimalActivity {
                kind: ActivityType::Listening,
                name: "your pins!".to_string(),
                url: None,
            }
            .into()],
            afk: false,
            since: None,
            status: Status::Online,
        }
    }
}

pub mod state_consts {
    pub const QUEUE_LEN: usize = 5;
}
