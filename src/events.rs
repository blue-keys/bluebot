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
                }
            }
        }
    }
}