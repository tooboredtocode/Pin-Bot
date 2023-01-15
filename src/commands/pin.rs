/*
 * Copyright (c) 2022 tooboredtocode
 * All Rights Reserved
 */

use twilight_model::application::command::{Command, CommandType};
use twilight_util::builder::command::CommandBuilder;

pub const COMMAND_NAME: &str = "Pin";

pub fn command() -> Command {
    CommandBuilder::new(COMMAND_NAME, "", CommandType::Message)
        .dm_permission(false)
        .build()
}
