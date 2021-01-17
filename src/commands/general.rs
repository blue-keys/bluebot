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
