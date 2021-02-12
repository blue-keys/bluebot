use super::core::{ext_lib::*, our_lib::*};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx : Context, _ready : Ready) {
        println!("{}", UTILS.events.ready_msg);
    }

    async fn guild_member_addition(&self, ctx: Context, guild_id : GuildId, mut new_member : Member) {
        if guild_id.as_u64() == &803919224539578378 {
            if new_member.user.id.as_u64() == &346637028118757378 {
                if new_member.add_role(&ctx, 803919224593711170).await.is_err() {
                    eprintln!("{}", UTILS.events.azales_admin_error);
                } else {
                    println!("{}", UTILS.events.azales_admin);
                    if ctx.http.send_message(803919224979980334, &"Le meilleur admin `Azales` est de retour !".to_string().into()).await.is_err() {
                        eprintln!("{}", UTILS.events.azales_libre_msg_error);
                    }
                }
            }
        }
    }

    async fn guild_member_removal(&self, ctx: Context, guild_id : GuildId, user : User, _ : Option<Member>) {
        if guild_id.as_u64() == &803919224539578378 {
            if user.id.as_u64() == &346637028118757378 {
                if ctx.http.send_message(803919224979980334, &"`Azales` reviendra dans une ou deux années plutonienne !".to_string().into()).await.is_err() {
                    eprintln!("{}", UTILS.events.azales_libre_msg_error);
                }
            }
        }
    }
}