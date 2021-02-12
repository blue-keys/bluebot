pub mod config;
pub mod support_commands;

pub mod ext_lib {

    pub use serenity::{
        framework::standard::{
            macros::{command, group, help, hook},
            CommandResult,
            CommandGroup,
            HelpOptions,
            Args,
            help_commands, 
            StandardFramework,
        },
        http::Http,
        model::{channel::Message, gateway::Ready, id::{UserId, GuildId}, guild::Member, user::User},
        client::{Context, Client, EventHandler, bridge::gateway::ShardManager},
        async_trait,
        utils::{Color, Colour},
        prelude::{Mutex, TypeMapKey},
    };

    pub use serde::Deserialize;

    pub use ron::from_str;
}


pub mod std_lib {

    pub use std::fs;
    pub use std::env;
    pub use std::collections::HashSet;
    pub use std::sync::Arc;
}

pub mod our_lib {

    pub use super::config::{
        strings::UTILS,
        bot_config::*,
    };

    pub use super::support_commands::*;

}

