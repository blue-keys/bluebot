use crate::core::{ext_lib::*, std_lib::*};

#[help]
#[individual_command_tip =
"Entrez la commande en argument pour avoir plus de détails la concernant"]
#[no_help_available_text("**Erreur** : Pas d'aide disponible")]
#[command_not_found_text = "**Erreur** : Commande introuvable"]
#[lacking_permissions = "Hide"]
#[lacking_role = "Hide"]
#[wrong_channel = "Hide"]
#[strikethrough_commands_tip_in_guild("")]
#[strikethrough_commands_tip_in_dm("")]
#[guild_only_text("Utilisable uniquement sur un serveur")]
#[dm_only_text("Utilisable uniquement en message privé")]
#[dm_and_guild_text("Utilisable partout")]
#[usage_sample_label("Exemple(s) : ")]
#[usage_label("Utilisation : ")]
#[grouped_label("Groupe : ")]
#[available_text("Disponible :")]
#[embed_success_colour(DARK_GREEN)]
#[embed_error_colour(DARK_RED)]
async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[hook]
pub async fn unknown_command(_ctx : &Context, _msg : &Message, unknown_command_name : &str) {
    println!("Impossible de trouver la commande : {}", unknown_command_name);
}