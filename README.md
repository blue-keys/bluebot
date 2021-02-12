# Socle Discord Bot

## Les liens utiles

Guide pour apprendre le langage rust : [Guide du Rust](https://docs.digitall.zone/guide/dev/langages-informatique/rust)

Documentation serenity : [serenity version 0.9.0-rc.1](https://docs.rs/serenity/0.9.0-rc.1/serenity/index.html)

Qu'est ce que le RON ? : [RON](https://github.com/ron-rs/ron)

Feuille de rappel du Ron : [feuille de rappel RON](https://github.com/ron-rs/ron/wiki/Specification)

La document de la lib ron en rust : [ron crate](https://docs.rs/ron/0.6.4/ron/)

Pour ajouter votre bot à votre serveur : https://discord.com/oauth2/authorize?client_id=123456789012345678&scope=bot (il suffit de remplacer ce qui se trouve après client_id par l'id de votre bot et de lancer ce lien sur votre navigateur) 

## Comment utiliser notre socle de bot ?

Ce socle est prévu pour être très simple d'utilisation, il a été codé avec la lib serenity en version 0.9.0-rc.1

### Le fichier strings.ron

Ce fichier contient toutes les chaînes de caractères qui seront utilisées dans le code (Sauf celles des macros), il utilise la notation ron et est directement lié au fichier strings.rs (src/core/config/strings.rs) ce qui signifie que si vous rajoutez
un champ dans le fichier strings.ron vous devrez aussi le rajoutez dans le fichier strings.rs pour que ça fonctionne

C'est dans ce fichier que vous préciserez le nom de la variable d'environnement sur votre machine qui contient le token de votre bot

### Ajouter une commande

Pour ajouter une commande, il suffit de choisir dans quel groupe de commande vous voulez l'ajoutez (si vous en créez un nouveau n'oubliez pas de l'ajouter dans le fichier bot_config.rs (src/core/config/bot_config.rs) dans la fonction get_framework), une fois que votre groupe de commande est choisi il suffit tout simplement de suivre le modèle des commandes du fichier general.rs (src/commands/general.rs)

## Installation Linux (Ubuntu 20.04 LTS)

```bash
# installation du langage, plus Cargo
sudo apt install rustc
```

```bash
# editer le fichier du profile de l'utilisateur
nano .profile

# ajouter à la fin votre token
export BLUEBOT_TOKEN=O000000000000000000000000.000000000000000

# actualiser le fichier profile
source .profoile

# pour vérifier si la modification est prise en compte
echo BLUEBOT_TOKEN

# on exécute le bot pour le connecter à Discord
cargo run
```


