/*
 * Copyright (c) 2022 tooboredtocode
 * All Rights Reserved
 */

use std::error::Error;

use tracing::{error, warn};
use twilight_gateway::cluster::ClusterStartError;
use twilight_http::response::DeserializeBodyError;
use twilight_http::Error as TwilightHttpErr;
use twilight_validate::request::ValidationError;

use crate::context::state::ClusterState;
use crate::context::Ctx;

pub struct ShutDown;

pub trait Expectable<T>
where
    Self: Sized,
{
    fn expect_with(self, msg: &str) -> Result<T, ShutDown>;

    fn warn_with(self, msg: &str) -> Option<T>;

    fn expect_with_state(self, msg: &str, ctx: &Ctx) -> Result<T, ShutDown> {
        let res = self.expect_with(msg);
        if let Err(_) = res {
            ctx.set_state(ClusterState::Crashing);
        }

        res
    }
}

pub trait BlanketImpl: Error {}

impl BlanketImpl for TwilightHttpErr {}

impl BlanketImpl for ValidationError {}

impl BlanketImpl for DeserializeBodyError {}

impl BlanketImpl for ClusterStartError {}

impl BlanketImpl for hyper::Error {}

impl<T, E: BlanketImpl> Expectable<T> for Result<T, E> {
    fn expect_with(self, msg: &str) -> Result<T, ShutDown> {
        let err = match self {
            Ok(ok) => return Ok(ok),
            Err(e) => e,
        };

        error!(failed_with = err.to_string(), "{}", msg);

        Err(ShutDown)
    }

    fn warn_with(self, msg: &str) -> Option<T> {
        let err = match self {
            Ok(ok) => return Some(ok),
            Err(e) => e,
        };

        warn!(failed_with = err.to_string(), "{}", msg);

        None
    }
}
