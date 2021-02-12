use crate::core::{ext_lib::*, our_lib::*};

#[command]
#[description("Une commande qui renvoie Pong en cas de réussite")]
async fn ping(ctx : &Context, msg : &Message) -> CommandResult {
    msg.channel_id.say(&ctx, &UTILS.commands.ping).await?;
    Ok(())
}

#[command]
#[description("Donne une description du bot")]
async fn info(ctx : &Context, msg : &Message) -> CommandResult {
    msg.channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title(&UTILS.commands.info_title)
             .description(&UTILS.commands.info_desc)
             .footer(|f| {
                 f.text(&UTILS.commands.info_footer)
             })
             .color(Color::DARK_GREEN);
            if let Some(fields_tmp) = &UTILS.commands.info_fields {
                let mut fields : Vec<(String, String, bool)> = vec![];
                fields.clone_from(fields_tmp);
                e.fields(fields)
            } else {
                e
            }
        })
    }).await?;
    Ok(())
}

#[command]
#[required_permissions(ADMINISTRATOR)]
#[description("Eteint le bot")]
#[max_args(0)]
async fn shutdown(ctx : &Context, msg : &Message) -> CommandResult {
    let data = ctx.data.read().await;

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        msg.reply(ctx, &UTILS.commands.shutdown_success).await?;
        manager.lock().await.shutdown_all().await;
    } else {
        msg.reply(ctx, &UTILS.commands.shutdown_failure).await?;

        return Ok(());
    }

    Ok(())
}

#[command]
#[required_permissions(ADMINISTRATOR)]
#[description("Redémarre le bot")]
#[max_args(0)]
async fn restart(ctx : &Context, msg : &Message) -> CommandResult {
    msg.reply(ctx, &UTILS.commands.restart).await?;
    ctx.shard.shutdown_clean();

    Ok(())
}