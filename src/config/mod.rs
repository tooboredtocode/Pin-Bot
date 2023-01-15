/*
 * Copyright (c) 2022 tooboredtocode
 * All Rights Reserved
 */

use figment::providers::{Env, Format, Json, Yaml};
use figment::{Error, Figment};
use serde::Deserialize;

use crate::constants::config_consts;

pub mod discord;
pub mod logging;
pub mod metrics;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub discord: discord::Options,
    #[serde(default)]
    pub metrics: metrics::Options,
    #[serde(default)]
    pub logging: logging::Options,
}

impl Config {
    pub fn load() -> Result<Self, Error> {
        Figment::new()
            .merge(Yaml::file(config_consts::YAML_FILE_PATH))
            .merge(Json::file(config_consts::JSON_FILE_PATH))
            .join(Env::prefixed(config_consts::ENV_PREFIX).split("."))
            .extract()
    }
}
