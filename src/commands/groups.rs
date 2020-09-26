use crate::core::{ext_lib::*};

use super::{
    general::*,
};

#[group("General")]
#[description("Un groupe de commandes réunissant celles que l'on retrouve le plus généralement sur tous les bots")]
#[commands(ping, info)]
pub struct General;

