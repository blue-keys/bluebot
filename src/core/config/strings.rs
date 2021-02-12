
use lazy_static::lazy_static;

use crate::core::{std_lib::*, ext_lib::*};

#[derive(Deserialize)]
pub struct BotConf {
    pub prefix : String,
    pub expect_token : String,
    pub expect_client_new : String,
    pub var_token : String,
}

#[derive(Deserialize)]
pub struct Events {
    pub ready_msg : String,
    pub azales_admin_error : String,
    pub azales_admin : String,
    pub azales_libre_msg_error : String,
}

#[derive(Deserialize)]
pub struct Commands {
    pub ping : String,
    pub info_title : String,
    pub info_desc : String,
    pub info_fields : Option<Vec<(String, String, bool)>>,
    pub info_footer : String,
    pub shutdown_success : String,
    pub shutdown_failure : String,
    pub restart : String,
}

#[derive(Deserialize)]
pub struct Utils {
    pub bot_conf : BotConf,
    pub events : Events,
    pub commands : Commands,
}

lazy_static! {
    pub static ref UTILS : Utils = {
        let file = fs::read_to_string("strings.ron").unwrap();
        from_str(&file).unwrap()
    };
}

#[cfg(test)]
mod tests {
    use super::UTILS;

    #[test]
    fn verif_prefix() {
        assert_eq!("kecharrm ", &UTILS.bot_conf.prefix);
    }

    #[test]
    fn verif_ready_msg() {
        assert_eq!("Kecharrm est réveillé", &UTILS.events.ready_msg);
    }
}