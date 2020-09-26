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
        model::{channel::Message, gateway::Ready, id::UserId},
        client::{Context, Client, EventHandler},
        async_trait,
        utils::{Color, Colour},
    };

    pub use serde::Deserialize;

    pub use ron::from_str;
}


pub mod std_lib {

    pub use std::fs;
    pub use std::env;
    pub use std::collections::HashSet;

}

pub mod our_lib {

    pub use super::config::{
        strings::UTILS,
        bot_config::*,
    };

    pub use super::support_commands::*;

}

